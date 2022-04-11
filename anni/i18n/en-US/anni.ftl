## anni
anni-about = A set of tools for self-hosting music sites.
export-to = Path for exported data.


## flac
flac = Provide FLAC-related utilities.
flac-export = Export data.
flac-export-type = Type of data to export.


## split
split = Provided helper to split audios.
split-format-input = Format of input audio file.
split-format-output = Format of output audio file.
split-no-apply-tags = Do not extract tags from CUE files to audio file.
split-no-import-cover = Do not import cover to audio file.
split-output-file-exist = Output file {$filename} exists. Please remove the file and try again.


## convention
## FIXME
convention = Provided audio convention utilities.
convention-check = Check audio.
convention-check-fix = Apply fixes to audio files directly.


## repo
repo = Anni metadata repository manager.
repo-root = Root path of Anni metadata repository to manage.

repo-clone = Clone metadata repository.
repo-clone-start = Cloning metadata repository to {$path}...
repo-clone-done = Metadata repository cloned.

repo-add = Add new album to repository.
repo-add-edit = Open text editor after album metadata file is crated.
repo-invalid-album = Invalid album folder name format: {$name}
repo-album-exists = Album with catalog {$catalog} already exists in repo.
repo-album-not-found = Album with catalog {$catalog} was not found in repo.
repo-album-info-mismatch = Album info mismatch with folder.

repo-import = Import album to repository.
repo-import-format = Format of album metadata file.

repo-validate-start = Start validating repository.
repo-validate-end = End validating repository.
repo-validate-failed = Validation failed.
repo-catalog-filename-mismatch = Album catalog '{$album_catalog}' does not match filename.
repo-invalid-artist = Invalid artist: {$artist}

repo-get = Get album metadata from remote data source and add to repo.
repo-get-print = Print album metadata to stdout.
repo-get-cue-keyword = If metadata is not enough, search vgmdb with keyword.
repo-get-cue-catalog = Specify catalog when it does not exist.
repo-cue-insufficient-information = Insufficient information from CUE file.

repo-edit = Open text editor for an album if metadata exists.
repo-apply = Apply metadata from repository to album.
repo-validate = Check whether data in repository is valid.

repo-print = Print metadata information of given catalog.
repo-print-type = Print type.
repo-print-clean = Do not print REM COMMENT "Generated by Anni" in cue mode.
repo-print-catalog = Catalog to print. '/{"{disc_id}"}' can be appended to indicate the disc id of an album. Disc id equals to 0 or 1 both indicates the first disc.

repo-db = Generate metadata database from repository.

repo-migrate = Migrate metadata repository to new version.
repo-migrate-album-id = Add album_id field to album metadata.


## Library
library = Anni Audio library manager.
