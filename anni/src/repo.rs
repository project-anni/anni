use anni_repo::album::{Track, Disc};
use clap::ArgMatches;
use anni_repo::structure::{album_info, disc_info, file_name};
use anni_repo::{Album, RepositoryManager};
use anni_utils::fs;
use std::path::{PathBuf, Path};
use shell_escape::escape;
use anni_flac::FlacHeader;

pub(crate) fn handle_repo(matches: &ArgMatches) -> anyhow::Result<()> {
    let settings = RepositoryManager::new(matches.value_of("repo.root").unwrap())?;

    if let Some(matches) = matches.subcommand_matches("apply") {
        handle_repo_apply(matches, &settings)?;
    } else if let Some(matches) = matches.subcommand_matches("edit") {
        handle_repo_edit(matches, &settings)?;
    } else if let Some(matches) = matches.subcommand_matches("add") {
        handle_repo_add(matches, &settings)?;
    } else {
        unimplemented!();
    }
    Ok(())
}

fn handle_repo_add(matches: &ArgMatches, settings: &RepositoryManager) -> anyhow::Result<()> {
    let to_add = Path::new(matches.value_of("Filename").unwrap());
    let last = anni_repo::structure::file_name(to_add)?;
    if last.ends_with("]") {
        bail!("You can only add a valid album directory in anni convention to anni metadata repository.");
    }

    let (release_date, catalog, title) = album_info(&last)?;
    if settings.album_exists(&catalog) {
        bail!("Album with the same catalog exists in repo. Aborted.");
    }

    let mut album = Album::new(&title, "Artist", release_date, &catalog);

    let directories = fs::get_subdirectories(to_add)?;
    let mut directories: Vec<_> = directories.iter().map(|r| r.as_path()).collect();
    let mut has_discs = true;
    if directories.len() == 0 {
        directories.push(to_add);
        has_discs = false;
    }

    for dir in directories.iter() {
        let files = fs::get_ext_files(PathBuf::from(dir), "flac", false)?.unwrap();
        let mut disc = if has_discs {
            let (catalog, _, _) = disc_info(&*file_name(dir)?)?;
            Disc::new(&catalog)
        } else {
            Disc::new(&catalog)
        };
        for path in files.iter() {
            let header = FlacHeader::from_file(path)?;
            let track = stream_to_track(&header);
            disc.add_track(track);
        }
        album.add_disc(disc);
    }

    let file = settings.with_album(&catalog);
    fs::write(&file, album.to_string())?;
    if matches.is_present("edit") {
        edit::edit_file(&file)?;
    }
    Ok(())
}

fn handle_repo_edit(matches: &ArgMatches, settings: &RepositoryManager) -> anyhow::Result<()> {
    let to_add = Path::new(matches.value_of("Filename").unwrap());
    let last = anni_repo::structure::file_name(to_add)?;
    if last.ends_with("]") {
        bail!("You can only add a valid album directory in anni convention to anni metadata repository.");
    }

    let (_, catalog, _) = album_info(&last)?;
    if !settings.album_exists(&catalog) {
        bail!("Catalog not found in repo. Aborted.");
    }
    let file = settings.with_album(&catalog);
    edit::edit_file(&file)?;
    Ok(())
}

fn handle_repo_apply(matches: &ArgMatches, settings: &RepositoryManager) -> anyhow::Result<()> {
    let to_apply = Path::new(matches.value_of("Filename").unwrap());
    let last = anni_repo::structure::file_name(to_apply)?;
    if last.ends_with("]") {
        bail!("You can only apply album metadata to a valid anni convention album directory.");
    }

    let (release_date, catalog, album_title) = album_info(&last)?;
    if !settings.album_exists(&catalog) {
        bail!("Catalog not found in repo. Aborted.");
    }

    let album = settings.load_album(&catalog)?;
    if album.title() != album_title || album.catalog() != catalog || album.release_date() != &release_date {
        bail!("Album info mismatch. Aborted.");
    }

    let discs = album.discs();
    for (i, disc) in album.discs().iter().enumerate() {
        let disc_num = i + 1;
        let disc_dir = if discs.len() > 1 {
            to_apply.join(format!("[{}] {} [Disc {}]", disc.catalog(), album_title, disc_num))
        } else {
            to_apply.to_owned()
        };
        let files = fs::get_ext_files(disc_dir, "flac", false)?.unwrap();
        let tracks = disc.tracks();
        if files.len() != tracks.len() {
            bail!("Track number mismatch in Disc {} of {}. Aborted.", disc_num, catalog);
        }

        for i in 0..files.len() {
            let file = &files[i];
            let track = &tracks[i];
            let meta = format!(r#"TITLE={}
ALBUM={}
ARTIST={}
DATE={}
TRACKNUMBER={}
TRACKTOTAL={}
DISCNUMBER={}
DISCTOTAL={}"#, track.title(), album_title, track.artist(), album.release_date(), i + 1, tracks.len(), disc_num, discs.len());
            println!("echo {} | metaflac --remove-all-tags --import-tags-from=- {}", escape(meta.into()), escape(file.to_str().unwrap().into()));
        }
    }
    Ok(())
}

pub(crate) fn stream_to_track(stream: &FlacHeader) -> Track {
    let comment = stream.comments().unwrap();
    let map = comment.to_map();
    Track::new(map["TITLE"].value(), Some(map["ARTIST"].value()), None)
}
