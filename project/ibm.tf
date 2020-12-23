variable "ibmcloud_api_key" {}
variable "ibmcloud_washington_namespace" {}

terraform {
  required_providers {
    ibm = {
      source  = "IBM-Cloud/ibm"
      version = "~> 1.12.0"
    }
  }
}

provider "ibm" {
  ibmcloud_api_key = var.ibmcloud_api_key
  region           = "us-east"
}

resource "ibm_function_action" "fetch_prices_rs" {
  name      = "fetch_prices_rs"
  namespace = var.ibmcloud_washington_namespace

  exec {
    kind  = "Blackbox"
    image = "openwhisk/dockerskeleton"
    code  = filebase64("target/fetch_prices.zip")
  }

  limits {
    timeout = 10000
    memory  = 128
  }
}

resource "ibm_function_action" "fetch_prices_js" {
  name      = "fetch_prices_js"
  namespace = var.ibmcloud_washington_namespace

  exec {
    kind = "nodejs:12"
    code = file("dist/fetch_prices.bundle.js")
  }

  limits {
    timeout = 10000
    memory  = 128
  }
}

