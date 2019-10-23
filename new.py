import argparse
import os

import archive_lib


def main(arguments):
    full_name = archive_lib.get_name()

    tmp_path = arguments['tmpPath']
    os.chdir(tmp_path)
    os.mkdir(full_name)

    os.chdir(full_name)

    for sub_dir in ['mp3', 'wav', 'img', 'doc']:
        os.mkdir(sub_dir)

    path = 'info.txt'
    with open(path, 'a'):
        os.utime(path, None)

    print "-- done preparing: %s" % full_name


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Open a new item')
    parser.add_argument('tmpPath')
    parser.add_argument('-v', help='verbose', action='store_true')

    print
    main(vars(parser.parse_args()))
    print
