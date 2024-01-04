FROM python:3-slim
RUN pip install --upgrade pip
RUN pip install requests tomli packaging

COPY version_checker.py /version_checker.py

ENTRYPOINT ["python3", "/version_checker.py"]