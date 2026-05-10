from boxer_validator_airflow._core import hello_from_rust


def hello() -> str:
    return hello_from_rust()
