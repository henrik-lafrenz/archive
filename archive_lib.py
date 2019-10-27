import os
import re


ARCHIVE_BASE = 'ARK'
ARCHIVE_SUB = 'ARCHIVE'


class ArchivePathError(Exception):
    def __init__(self, message):
        self.message = message


def verify_archive_path(path):
    archive_base = os.path.abspath(path)
    
    if not os.path.exists(archive_base):
      raise ArchivePathError('invalid path')
    
    if not os.path.split(archive_base)[-1] == ARCHIVE_BASE:
      raise ArchivePathError('did not find %s' % ARCHIVE_BASE)
    
    archive_sub = os.path.join(archive_base, ARCHIVE_SUB)
    if not os.path.exists(archive_sub):
        raise ArchivePathError('did not find %s' % ARCHIVE_SUB)

    return archive_sub 

# found on stack overflow, by Fred Cirera
def sizeof_fmt(num):
    for x in ['bytes','KB','MB','GB','TB']:
        if num < 1024.0:
            return "%3.1f%s" % (num, x)
        num /= 1024.0


def is_ignored(filename):
    return os.path.basename(filename).startswith('.') \
        or filename.startswith('__MACOSX')


def get_name():
    item_date = str.strip(input("new date: "))
    if len(item_date) == 0:
        print("invalid item date")
        return

    item_artist = str.strip(input("new artist: "))
    if len(item_artist) == 0:
        print("invalid item artist")
        return

    item_title = str.strip(input("new title: "))
    if len(item_title) == 0:
        print("invalid item title")
        return

    item_location = str.strip(input("new location: "))
    if len(item_location) == 0:
        print("invalid item location")
        return

    return '%s %s - %s - %s' % (item_date, item_artist, item_title, item_location)


def find_items(archive_path, arguments):
    compiled_re = re.compile(('.*%(match)s.*' % arguments), flags=re.IGNORECASE) if arguments['match'] else None
    file_names = []

    end_with_lb = False

    for file_name in sorted(os.listdir(archive_path)):
        if file_name.startswith('.'):
            continue

        if compiled_re and not compiled_re.match(file_name):
            continue

        if file_name[-4:] != '.zip':
            print("ignoring non-zip file: %s" % file_name)
            end_with_lb = True
            continue

        file_names.append(file_name[:-4])

    if end_with_lb:
        print

    return file_names
