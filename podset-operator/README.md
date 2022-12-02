# podset operator

``` bash
operator-sdk init --project-name job-operator --domain=example.com --repo=github.com/mohammadne/sandbox/podset-operator
operator-sdk create api --group=app --version=v1alpha1 --kind=PodSet --controller --resource

# 
vim api/v1alpha1/podset_type.go

```
