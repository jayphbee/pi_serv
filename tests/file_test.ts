import { File } from './native/pi_serv_builtin/file'

const name = 'test.txt';
const nameCopy = 'test_copy.txt';
// 文件测试
const test = async () => {
    // 判断文件是否存在
    if (File.fs_access(name)) {
        // 删除文件
        await File.fs_unlink(name);
    }
    // 判断文件是否存在
    if (File.fs_access(nameCopy)) {
        // 删除文件
        await File.fs_unlink(nameCopy);
    }
    // 创建文件
    if (await File.fs_write_file(name, str2ab('test!!!!!!!!!!')) !== 14) throw new Error();

    // 追加文件内容
    if (await File.fs_append_file(name, str2ab('test2222222222222')) !== 17) throw new Error();
    // 读取文件状态
    if (File.fs_stat_size(name) !== 31) throw new Error();
    // 读取文件内容
    if (await File.fs_read_file_string(name, 0, 0) !== 'test!!!!!!!!!!test2222222222222') throw new Error();
    // 读取文件内容
    if (await (await File.fs_read_file_binary(name, 0, 0)).byteLength !== 31) throw new Error();
    // 复制文件
    await File.fs_copy_file(name, name + 'copy');
    // 获取目录列表
    if (!File.fs_readdir('./')) throw new Error();
    // 创建目录
    if (!await File.fs_mkdir('./testdir')) throw new Error();
    // 创建目录
    if (!await File.fs_mkdir('./testdir2')) throw new Error();
    // 删除目录
    if (!await File.fs_rmdir('./testdir2')) throw new Error();
}

// test().then(() => {
//     console.log('!!!!!!!!!!!!test ok!!');
// }).catch((e) => {
//     console.log('!!!!!!!!!test error:', e);
// })

const str2ab = (str: string): Uint8Array => {
    const arr = [];
    for (let i = 0, strLen = str.length; i < strLen; i++) {
        arr[i] = str.charCodeAt(i);
    }

    return new Uint8Array(arr);
};