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

CREATE TABLE test_table
(
   [id] int IDENTITY(1, 1)  NOT NULL,
   [createdAt] datetime2  NOT NULL,
   [updatedAt] datetime2  NOT NULL,
   [name] varchar(150)  NOT NULL,
   [amount] decimal(15, 2)  NULL
)
WITH (DATA_COMPRESSION = NONE)
GO

ALTER TABLE test_table ADD CONSTRAINT [PK_test_table] PRIMARY KEY CLUSTERED ([id] ASC)
GO

INSERT INTO test_table ([createdAt],[updatedAt],[name],[amount]) VALUES (GETDATE(), GETDATE(), 'test name 1', 1.0);
INSERT INTO test_table ([createdAt],[updatedAt],[name],[amount]) VALUES (GETDATE(), GETDATE(), 'test name 2', 1.0);
INSERT INTO test_table ([createdAt],[updatedAt],[name],[amount]) VALUES (GETDATE(), GETDATE(), 'test name 3', 1.0);
INSERT INTO test_table ([createdAt],[updatedAt],[name],[amount]) VALUES (GETDATE(), GETDATE(), 'test name 4', 1.0);
INSERT INTO test_table ([createdAt],[updatedAt],[name],[amount]) VALUES (GETDATE(), GETDATE(), 'test name 5', 1.0);
GO
