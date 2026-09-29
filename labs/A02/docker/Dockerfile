FROM python:3.11-slim

WORKDIR /app

RUN pip install --no-cache-dir flask==3.0.3

COPY . .

EXPOSE 8012

CMD ["python", "app.py"]
