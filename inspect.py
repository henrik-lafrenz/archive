import argparse
import os
import re
import sys
import zipfile

import archive_lib


def main(arguments):
	try:
		archive_path = archive_lib.verify_archive_path(arguments['archivePath'])
		file_names = find_items(archive_path, arguments)

		if len(file_names) == 0:
			print "no items found.\n"
		elif len(file_names) == 1:
			if arguments['i']:
				print_info(file_names[0], archive_path)
			else:
				print_item(file_names[0], archive_path)
		else:
			print_items(file_names)

	except archive_lib.ArchivePathError, e:
		print "path error: %s" % e. message


def split_file_name(file_name):
	date = file_name[:10]
	name = file_name[11:]
	try:
		artist, title, location = name.split(' - ')
	except ValueError:
		print "problem with item name: %s.zip" % file_name

	return (date, artist, title, location)


def print_info(file_name, archive_path):
	print_file_name_data(file_name)

	path = os.path.join(archive_path, file_name + ".zip")
	info_file_name = None
	with zipfile.ZipFile(path) as zf:
		for member in zf.infolist():
			if (os.path.basename(member.filename) in ['info.rtf', 'info.txt']):

				info_file_name = member.filename 
				content = zf.read(info_file_name)
				for byte in content:
					sys.stdout.write(byte)
				print
				return

	print "no info file found."


def print_item(file_name, archive_path):
	print_file_name_data(file_name)

	path = os.path.join(archive_path, file_name + ".zip")
	with zipfile.ZipFile(path) as zf:
		for member in zf.infolist():
			if not archive_lib.is_ignored(member.filename):
				print "-- %s" % member.filename


def print_file_name_data(file_name):
	date, artist, title, location = split_file_name(file_name)
	print "date: %s" % date
	print "artist: %s" % artist
	print "title: %s" % title
	print "location: %s\n" % location


def print_items(file_names):
	print "| date       | artist                         | title                                    | location                       |"     
	print "+------------+--------------------------------+------------------------------------------+--------------------------------+"
	for file_name in file_names:
		date, artist, title, location = split_file_name(file_name)
		print "| %s | %-30s | %-40s | %-30s |" % (date, artist[:30], str(title)[:40], location[:30])


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
			print "ignoring non-zip file: %s" % file_name
			end_with_lb = True
			continue

		file_names.append(file_name[:-4])

	if end_with_lb:
		print

	return file_names


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Inspect the archive')
    parser.add_argument('archivePath')
    parser.add_argument('--match')
    parser.add_argument('-i', help='print info', action='store_true')
    
    print
    main(vars(parser.parse_args()))
    print
    