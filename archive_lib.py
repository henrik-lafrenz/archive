import os


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
