#!/usr/bin/env bash

# Launch MSSQL and send to background
/opt/mssql/bin/sqlservr &
pid=$!

sleep 30

is_up=1

while [ $is_up -ne 0 ]
do
  echo -e $(date)
  /opt/mssql-tools/bin/sqlcmd -l 60 -S localhost -U sa -P $SA_PASSWORD -h-1 -V1  -Q "SET NOCOUNT ON SELECT \"Server ready\" , @@servername"
  is_up=$?
  sleep 5
done

/opt/mssql-tools/bin/sqlcmd -l 60 -S localhost -U sa -P $SA_PASSWORD -i ./init-scripts/create-database.sql

wait $pid
