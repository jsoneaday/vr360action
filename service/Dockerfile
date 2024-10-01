# Use the official PostgreSQL image as a base
FROM postgres:15

# Set environment variables for PostgreSQL
ENV POSTGRES_DB=mydatabase
ENV POSTGRES_USER=myuser
ENV POSTGRES_PASSWORD=mypassword

# Expose the default PostgreSQL port
EXPOSE 5432

# Copy initialization scripts (optional)
# COPY ./init.sql /docker-entrypoint-initdb.d/

# The official PostgreSQL image will handle the rest
