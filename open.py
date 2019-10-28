import argparse
import os

import archive_lib


def main(arguments):
    try:
        archive_path = archive_lib.verify_archive_path(arguments['archivePath'])
        item_path = archive_lib.find_item(archive_path, arguments)
        if not item_path:
            return

        archive_lib.copy_to_tmp(arguments['tmpPath'], item_path)
        (tmp_file, unzipped_folder_name) = archive_lib.unzip_item(arguments['tmpPath'], item_path)
        archive_lib.delete_tmp_zip(tmp_file)

        print("extracted '%s' to '%s'" % (unzipped_folder_name, arguments['tmpPath']))
        print("done.")

    except archive_lib.ArchivePathError as e:
        print("path error: %s" % e. message)
    

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Open an item to a temporary location')
    parser.add_argument('archivePath')
    parser.add_argument('tmpPath')
    parser.add_argument('match')

    print
    main(vars(parser.parse_args()))
    print
