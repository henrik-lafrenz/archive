import argparse
import os
import sys
import zipfile

import archive_lib


def main(arguments):
    try:
        archive_path = archive_lib.verify_archive_path(arguments['archivePath'])
        file_names = archive_lib.find_items(archive_path, arguments)

        if len(file_names) == 0:
            print("no items found.\n")
        elif len(file_names) == 1:
            if arguments['i']:
                print_info(file_names[0], archive_path)
            else:
                print_item(file_names[0], archive_path)
        else:
            print_items(file_names)

    except archive_lib.ArchivePathError as e:
        print("path error: %s" % e. message)


def split_file_name(file_name):
    date = file_name[:10]
    name = file_name[11:]
    try:
        artist, title, location = name.split(' - ')
        return (date, artist, title, location)
    except ValueError:
        print("problem with item name: %s.zip" % file_name)
        return None


def print_info(file_name, archive_path):
    print_file_name_data(file_name)

    path = os.path.join(archive_path, file_name + ".zip")
    info_file_name = None
    with zipfile.ZipFile(path) as zf:
        for member in zf.infolist():
            basename = os.path.basename(member.filename)
            if basename in ['info.rtf', 'info.txt']:
                print("%s:" % basename)
                print('\n' + '-' * 80 + '\n')
                print(zf.read(member.filename).decode('utf-8'))
                print('\n' + '-' * 80 + '\n')
                return

    print("no info file found.")


def print_item(file_name, archive_path):
    print_file_name_data(file_name)

    path = os.path.join(archive_path, file_name + ".zip")
    with zipfile.ZipFile(path) as zf:
        for member in zf.infolist():
            if not archive_lib.is_ignored(member.filename):
                print("-- %s" % member.filename)


def print_file_name_data(file_name):
    res = split_file_name(file_name)
    if not res:
        print("problem with item name: %s.zip" % file_name)

    date, artist, title, location = res
    print("date: %s" % date)
    print("artist: %s" % artist)
    print("title: %s" % title)
    print("location: %s\n" % location)


def print_items(file_names):
    print("| date       | artist                         | title                                    | location                       |"     )
    print("+------------+--------------------------------+------------------------------------------+--------------------------------+")
    for file_name in file_names:
        res = split_file_name(file_name)
        if not res:
            return

        date, artist, title, location = res
        print("| %s | %-30s | %-40s | %-30s |" % (date, artist[:30], str(title)[:40], location[:30]))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Inspect the archive')
    parser.add_argument('archivePath')
    parser.add_argument('--match')
    parser.add_argument('-i', help='print info', action='store_true')

    print
    main(vars(parser.parse_args()))
    print
