import os
import boto3
import json

s3 = boto3.client('s3')

def response_generator(statusCode=200, message=""):
    return {
        'statusCode': statusCode,
        'body': {
            'message': message
        },
        "headers": {
            "Content-Type": "application/json"
        }
    }
    
    
def lambda_handler(event, context):
    
    S3_BUCKET_NAME = os.environ.get('S3_BUCKET_NAME')
    
    body = json.loads(event.get('body'))
    
    event = body
    file = event.get('file')
    name = event.get('name')
    
    print(file)
    print(name)
    
    try:
        s3.put_object(Body=file, Bucket=S3_BUCKET_NAME, Key=name)
    except Exception as e:
        print(e)
        return response_generator(400, str(e))
    
    
    return 200
    pass