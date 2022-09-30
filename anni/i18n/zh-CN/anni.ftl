## anni
anni-about = 为自建音乐站点构建的一整套工具
export-to = 导出内容存放的路径


## flac
flac = 提供 FLAC 处理相关的功能
flac-export = 导出内容
flac-export-type = 导出内容类型


## split
split = 提供音频分割相关的功能
split-format-input = 待切分音频的文件类型
split-format-output = 切分后输出音频的文件类型
split-clean = 不向切分后的音频文件中写入元数据和封面等信息
split-no-import-cover = 不从切分目录寻找封面写入音频文件
split-output-file-exist = 输出路径下已存在文件 {$filename}，请删除文件后重试


## convention
convention = 提供定制化的音频检查约定检测
convention-check = 检查音频是否符合约定
convention-check-fix = 对不符合约定的音频文件进行修复


## repo
repo = 提供 Anni 元数据仓库的管理功能
repo-root = 需要管理的 Anni 元数据仓库根路径

repo-clone = 克隆元数据仓库
repo-clone-start = 准备克隆元数据仓库至{$path}...
repo-clone-done = 元数据仓库克隆完成

repo-add = 向元数据仓库中导入专辑
repo-add-edit = 在导入完成后打开文件编辑器
repo-invalid-album = 专辑目录格式错误：{$name}
repo-album-exists = 专辑 {$catalog} 已存在
repo-album-not-found = 不存在品番为 {$catalog} 的专辑
repo-album-info-mismatch = 专辑信息与专辑目录不一致

repo-import = 导入专辑
repo-import-format = 导入专辑的数据格式

repo-validate-start = 仓库校验开始
repo-validate-end = 仓库校验结束
repo-validate-failed = 仓库校验失败
repo-catalog-filename-mismatch = 专辑 {$album_catalog} 的品番与文件名不一致
repo-invalid-artist = 艺术家名称不可用：{$artist}

repo-get = 从远程数据源获取专辑信息并导入
repo-get-print = 将获取的专辑信息输出到控制台而非导入
repo-get-cue-keyword = 当元数据缺失时，使用该关键字搜索 VGMdb
repo-get-cue-catalog = 当 catalog 不存在时，手动指定
repo-cue-insufficient-information = CUE 文件未能提供足够的信息

repo-edit = 当元数据仓库中存在该专辑时，打开仓库中对应的文件
repo-validate = 检查仓库数据的合法性

repo-print = 根据品番输出元数据仓库中的数据
repo-print-type = 输出数据的类型
repo-print-clean = 省略 cue 输出中的 REM COMMENT "Generated by Anni"
repo-print-input = 需要输出的对象。可以是标签名称或专辑品番。当表示专辑品番时，可以通过get_albums_by_tag后缀 '/{"{disc_id}"}' 指定需要输出信息的碟片编号，0 和 1 均代表第一张碟片

repo-db = 生成元数据仓库对应的数据库文件

repo-migrate = 迁移旧版本元数据仓库到新版本
repo-migrate-album-id = 为缺少 album_id 字段的专辑添加这一字段


## Library
library = 提供音频仓库的管理功能
library-tag = 将元数据仓库中的数据应用到专辑
library-link = 以符号链接将约定目录格式转换为严格目录格式


## Workspace
workspace = 管理音频整理工作空间
workspace-init = 初始化工作空间
workspace-create = 在工作空间中创建新专辑

workspace-add = 将工作空间中专辑对状态从未跟踪转换为已跟踪
workspace-add-import-tags=从音频文件中导入元数据
workspace-add-dry-run=仅展示，不实际移动文件或创建链接
workspace-add-skip-check=跳过对专辑结构的检查

workspace-rm = 从工作空间中移除专辑
workspace-status = 显示工作空间中所有专辑的状态
workspace-update = 更新工作空间中的专辑
workspace-publish = 将工作空间中的专辑发布到音频仓库
workspace-fsck = 检查并修复工作空间


## Completions
completions = 生成 Shell 的补全脚本
completions-shell = 生成补全脚本的 Shell
