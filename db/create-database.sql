USE master
GO

IF NOT EXISTS(SELECT 1 FROM sys.server_principals WHERE name = N'test')
  CREATE LOGIN test WITH PASSWORD = '$(TEST_PASSWORD)'
GO

CREATE DATABASE test
GO

USE test
GO

IF NOT EXISTS(SELECT 1 FROM sys.database_principals WHERE name = 'test')
  CREATE USER test FOR LOGIN test;
GO

ALTER ROLE db_owner ADD MEMBER test;
GO

CREATE TABLE todos
(
   [id] int IDENTITY(1, 1)  NOT NULL,
   [description] varchar(500)  NOT NULL,
   [done] bit NOT NULL
)
WITH (DATA_COMPRESSION = NONE)
GO

ALTER TABLE todos ADD CONSTRAINT [PK_todos] PRIMARY KEY CLUSTERED ([id] ASC)
GO
