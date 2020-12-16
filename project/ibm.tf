variable "ibmcloud_api_key" {}

terraform {
  required_providers {
    ibm = {
      source = "IBM-Cloud/ibm"
      version = "~> 1.12.0"
    }
  }
}

provider "ibm" {
  ibmcloud_api_key = var.ibmcloud_api_key
  region           = "us-east"
}

resource "ibm_function_action" "fetch_prices" {
  name      = "fetch_prices"
  namespace = "washington"

  exec {
    kind  = "blackbox"
    image = "openwhisk/dockerskeleton"
    code  = filebase64("target/fetch_prices.zip")
  }

  limits {
    timeout = 10000
    memory  = 128
  }
}
