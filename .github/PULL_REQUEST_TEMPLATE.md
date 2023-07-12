## 🚨 WARNING
In case that your PR doesn't require a deployment, please add the label `no-deploy` to it.

**Example of no-deploy cases:**
- Documentation changes
- Code style changes
- Refactoring
- Linting
- Tests
- Dev dependencies updates

**Example of 'no-deploy' label:**
```bash	
refactor: linting queries. [no-deploy]
```	

The label should be added at the end of the PR title, between brackets.

Be SURE that your PR doesn't require a deployment before adding the label.

## Description

Add a short description of the PR changes.

Description:

## Affected Services

Check the services that will be affected by this PR.
- [ ] API
- [ ] Worker

In case that the PR only affects one of the services, please add the label `no-deploy-api` or `no-deploy-worker` to it accordingly.

**Example of 'no-deploy-api' label:**
```bash	
feat: new worker. [no-deploy-api]
```	

**Example of 'no-deploy-worker' label:**
```bash	
feat: new endpoint. [no-deploy-worker]
```	

The label should be added at the end of the PR title, between brackets.

## Zoom Link

Add the zoom link for the PR review meeting.

Link:


