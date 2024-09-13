import json
import boto3
import base64
import requests
import redis
import os

REDIS_ENDPOINT = os.environ.get('ELASTICACHE_ENDPOINT')
REDIS_PORT = os.environ.get("REDIS_PORT")


def get_redis_client():
    return redis.StrictRedis(
        host=REDIS_ENDPOINT,
        port=REDIS_PORT,
        socket_timeout=5,
        decode_responses=True
    )


def call_ec2_endpoint(ec2_payload):
    # Replace with your EC2 endpoint URL
    EC2_ENDPOINT_URL = os.environ.get('EC2_ENDPOINT_URL') + "/compile"
    print(EC2_ENDPOINT_URL)
    try:
        response = requests.post(
            EC2_ENDPOINT_URL, json=ec2_payload, timeout=10)
        response.raise_for_status()
        return response.json(), response.status_code
    except requests.RequestException as e:
        raise Exception(f"Error calling EC2 endpoint: {e}")


def lambda_handler(event, context):

    body = json.loads(event.get('body'))

    event = body

    key = event.get("key")
    base64_code = event.get("base64_code")
    functions = event.get("functions")
    compiler = event.get("compiler")
    args = event.get("args")

    print(event.get('body'))
    print(key, base64_code, functions, compiler, args)

    # Input validation
    if not key or not base64_code or not functions or not compiler:
        return {
            'statusCode': 400,
            'body': json.dumps('Invalid input'),
            'headers': {
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*'
                }
        }

    redis_client = get_redis_client()
    
    print("Connected to redis")

    try:
        # Check if the key exists in Redis
        cached_response = redis_client.get(key)

        if cached_response:
            # Key found in cache, return the cached response
            print("Redis response found")
            return {
                'statusCode': 200,
                'body': cached_response,
                'headers': {
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*'
                }
            }

        print("cached response not found")

        # Key not found in cache, call EC2 endpoint
        ec2_payload = {
            "base64_code": base64_code,
            "functions": functions,
            "compiler": compiler,
            "args": args
        }

        print("sending request to ec2")

        response_data, status_code = call_ec2_endpoint(ec2_payload)

        print(response_data)
        print(status_code)

        if status_code == 200:
            redis_client.setex(key, 3600, json.dumps(
                response_data))  # cache for 1 hour

            # Return the response data
            return {
                'statusCode': 200,
                'body': json.dumps(response_data),
                'headers': {
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*'
                }
            }
        else:
            # EC2 endpoint returned an error
            return {
                'statusCode': status_code,
                'body': json.dumps(response_data),
                'headers': {
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*'
                }
            }

    except redis.RedisError as e:
        return {
            'statusCode': 500,
            'body': json.dumps(f"Error interacting with Redis: {e}"),
            'headers': {
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*'
                }
        }
    except requests.RequestException as e:
        return {
            'statusCode': 500,
            'body': json.dumps(f"Error calling EC2 endpoint: {e}"),
            'headers': {
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*'
                }
        }
