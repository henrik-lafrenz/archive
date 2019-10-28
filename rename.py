import argparse
import math
import os
import re
import shutil
import zipfile

import archive_lib


def main(arguments):
    try:
        archive_path = archive_lib.verify_archive_path(arguments['archivePath'])

        item_path = archive_lib.find_item(archive_path, arguments)
        if not item_path:
            return

        new_item_name = get_new_name(item_path)
        if not new_item_name:
            return

        rename_item(item_path, new_item_name, arguments)

    except archive_lib.ArchivePathError as e:
        print("path error: %s" % e.message)


def get_new_name(item_path):
    item_file = os.path.basename(item_path)
    print("rename file and contained folder: %s" % item_file)

    new_full_name = archive_lib.get_name()

    if '%s.zip' % new_full_name == item_file:
        print("new name is identical to old name")
        return

    try:
        new_full_name_ascii = new_full_name.encode('ascii')
    except UnicodeEncodeError:
        print("only ascii please")
        return

    print("item will be renamed to: %s.zip" % new_full_name)
    if input("continue (y/n)? ").lower() == "y":
        return new_full_name


def rename_item(item_path, new_item_name, arguments):
    tmp_path = arguments['tmpPath']

    archive_lib.copy_to_tmp(tmp_path, item_path)

    (tmp_file, unzipped_folder_name) = archive_lib.unzip_item(tmp_path, item_path)

    archive_lib.delete_tmp_zip(tmp_file)

    print("-- renaming item folder")
    src = os.path.join(tmp_path, unzipped_folder_name)
    dst = os.path.join(tmp_path, new_item_name)
    os.rename(src, dst)

    print("-- zipping item")
    os.chdir(tmp_path)
    with zipfile.ZipFile(os.path.join(dst + '.zip'),
                         'w',
                         compression=zipfile.ZIP_DEFLATED,
                         allowZip64=True) as zf:
        for root, dirs, files in os.walk(new_item_name):
            if arguments['v']:
                if dirs:
                    print("-- in directory %s, containing %s:" % (root, dirs))
                else:
                    print("-- in directory %s:" % root)
            for filename in files:
                if archive_lib.is_ignored(filename):
                    if arguments['v']:
                        print("-- skipping: %s" % filename)
                    continue
                if arguments['v']:
                    print("-- zipping: %s" % filename)
                zf.write(os.path.join(root, filename))

    print("-- deleting renamed item folder")
    shutil.rmtree(dst)

    print("-- copying renamed item back into archive")
    archive_path = os.path.dirname(item_path)
    new_tmp_item = '%s.zip' % dst
    shutil.copy2(new_tmp_item, archive_path)

    print("-- removing tmp zip")
    os.remove(os.path.join(tmp_path, new_tmp_item))

    print("-- comparing file sizes")
    renamed_path = os.path.join(archive_path, new_item_name + '.zip')
    renamed_info = os.stat(renamed_path)
    original_info = os.stat(item_path)
    if arguments['v']:
        print("-- original: %s bytes, renamed: %s bytes" % (original_info.st_size, renamed_info.st_size))
    if math.fabs(original_info.st_size - renamed_info.st_size) > (original_info.st_size / 20):
        renamed_size = archive_lib.sizeof_fmt(renamed_info.st_size)
        original_size = archive_lib.sizeof_fmt(original_info.st_size)
        print("-- file sizes differ by more than 5%: %s (new) vs. %s (old)" % (renamed_size, original_size))
        if input("remove original item (y/n)? ").lower() != "y":
            print("clean up yourself then - done.")
            return

    print("-- removing original item")
    os.remove(item_path)

    print("done.")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Rename a zip file and its contained folder')
    parser.add_argument('archivePath')
    parser.add_argument('tmpPath')
    parser.add_argument('match')
    parser.add_argument('-v', help='verbose', action='store_true')

    print
    main(vars(parser.parse_args()))
    print
