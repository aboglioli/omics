#!/bin/bash
BACKUP_DIR=./backups
DAYS_TO_KEEP=14
FILE_SUFFIX=_backup.sql
DATABASE=omics
USER=admin

FILE=`date +"%Y%m%d%H%M"`${FILE_SUFFIX}

OUTPUT_FILE=${BACKUP_DIR}/${FILE}

pg_dump -U ${USER} ${DATABASE} -F p -f ${OUTPUT_FILE}

# gzip $OUTPUT_FILE

echo "${OUTPUT_FILE} was created:"
ls -l ${OUTPUT_FILE}

find $BACKUP_DIR -maxdepth 1 -mtime +$DAYS_TO_KEEP -name "*${FILE_SUFFIX}.gz" -exec rm -rf '{}' ';'
