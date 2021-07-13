message = 信息
exit = 退出
proceed = 继续
error = 错误
ok = 确定
closed = [已关闭]
name = 名称
date = 日期
description = 描述
topic_selection = 选择尝鲜分支
topic_selection_description = 如下是当前可用于测试的尝鲜分支列表。选中一个或多个尝鲜分支即可获得测试用更新，
    反选即可回滚软件包到稳定版本。请使用方向键浏览，并用回车键 (Enter) 选择分支。
topic_selection_closed_topic_warning = 检测到已关闭或已合并的尝鲜分支，ATM 将会把受影响的包回滚到稳定版本。

refresh_manifest = 正在下载分支信息……
refresh_apt = 正在下载软件包信息……
nothing = 无待办事项。
dpkg_error = dpkg 返回错误码：{$status}
no_stable_version = Notice: {$count} packages could not be downgraded to their stable versions.
install_count = 将安装 {$count} 个软件包
erase_count = 将卸载 {$count} 个软件包
update_count = 将升级或降级 {$count} 个软件包
package_path_error = 无法解析软件包路径。
disk_space_decrease = 该操作将使用 {$size} 存储空间。
disk_space_increase = 该操作将释放 {$size} 存储空间。
details = 详情
tx_title = 操作详情
tx_body = 将进行如下操作：
tx_hold = Kept Back: {$package} (No stable version)
tx_install = 安装：{$package} ({$version})
tx_upgrade = 升级：{$package} (至 {$version})
tx_downgrade = 降级：{$package} (至 {$version})
tx_erase = 卸载：{$package} ({$version})

pk_metered_network = You seem to be on a metered or celluar network.
    ATM will consume a large amount of network data during the transaction.
    Do you still wish to continue?
pk_battery = You seem to be on battery power.
    ATM will deplete the battery rather quickly during the transaction.
    It is recommended to plug in the power supply to prevent sudden power failure.
    Do you still wish to continue?
pk_inhibit_message = ATM 正在点钞
pk_dbus_error = Failed to connect to D-Bus system bus: {$error}
pk_comm_error_mid_tx = PackageKit daemon unexpectedly disconnected or crashed mid-transaction.
    Your system is likely in an inconsistent state and requires repairing.
    Please quit ATM and run `apt install -f` in your terminal to fix the problem.

    Error message: {$error}
pk_comm_error = Unable to communicate with the PackageKit daemon: {$error}
pk_tx_error = PackageKit daemon reported an error: {$error}
pk_comm_no_response = PackageKit daemon did not return a response.
pk_invalid_id = Package identifier "{$name}" is invalid. This is a bug, please report this issue to https://github.com/AOSC-Dev/atm/issues/new.

exe-title = Transaction In-Progress
exe-overall = Overall Progress:
exe_download = 正在下载 {$name} ……
exe-install = Installing {$name}...
exe_download_file_error = 无法下载：{$name}
exe_download_error = 无法下载文件
#exe_verify_error = 校验出错：{$name}
#exe_path_error = 未知文件名：{$name}
#exe_batch_error = 无法下载软件包

apt_finished = APT 配置信息更新成功。
install_error = 安装软件包时发生错误：{$error}

#press_enter_to_return = 请按 Enter 键返回主菜单。
press_enter_to_bail = 请按 Enter 键退出程序。
