---
title: Amazon Web Services (AWS)
type: entity
id: entities/aws
tags:
- cloud
- infrastructure
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
---

## Overview

Amazon Web Services (AWS) is Amazon.com's cloud-computing platform. Launched publicly in 2006 with S3 and EC2, it has grown into a market-leading cloud provider with services spanning compute, storage, databases, messaging, analytics, and machine learning across more than 30 geographic regions.

## Characteristics

- Pay-as-you-go billing.
- Compute services: EC2, Lambda, Fargate, ECS, EKS.
- Storage: S3, EBS, EFS.
- Databases: DynamoDB, RDS, Aurora, Redshift.
- Messaging/streaming: SQS, SNS, Kinesis, MSK.

## Common Strategies

- Region/Availability-Zone topology for fault isolation.
- Managed services with deep integration via IAM and CloudWatch.
- Reserved capacity, savings plans, and spot instances for cost optimization.

## Related Entities

- [[entities/aws-lambda]]
- [[entities/dynamodb]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
