FROM mcr.microsoft.com/mssql/server:2017-latest

ENV ACCEPT_EULA=Y
ENV SA_PASSWORD=sa_p4ssword
ENV TEST_PASSWORD=test_p4ssword

COPY ./boot.sh /bin/boot.sh
COPY ./*.sql /init-scripts/
CMD sh /bin/boot.sh
