FROM python:3.11-alpine
WORKDIR /app
RUN pip install --no-cache-dir flask
COPY . /app/
EXPOSE 8080
CMD ["python", "app.py"]
