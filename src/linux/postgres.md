# postgres

```
sudo -iu postgres
initdb -D /var/lib/postgres/data
exit
```
```
sudo systemctl start postgresql
sudo systemctl enable postgresql
```
```
psql -U postgres
\password
```
```
SHOW hba_file;

sudo nano /var/lib/postgres/data/pg_hba.conf
```

scram-sha-256

```
sudo systemctl restart postgresql
```
```
psql -U postgres 

CREATE DATABASE crm_db OWNER postgres;

\c crm_db

\i base.sql
```
```
psql -U postgres -d crm_db
```