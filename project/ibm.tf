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

data "archive_file" "fetch_prices_js" {
  type        = "zip"
  output_path = "${path.module}/files/fetch_prices_js.zip"

  source {
    content  = file("dist/fetch_prices.bundle.js")
    filename = "index.js"
  }
}

data "archive_file" "fetch_prices_rs" {
  type        = "zip"
  output_path = "${path.module}/files/fetch_prices_rs.zip"

  source {
    content  = file("target/x86_64-unknown-linux-musl/release/fetch_prices")
    filename = "exec"
  }

  source {
    content  = file(".env")
    filename = ".env"
  }
}

resource "ibm_function_action" "fetch_prices_rs" {
  name      = "fetch_prices_rs"
  namespace = var.ibmcloud_washington_namespace

  exec {
    kind  = "Blackbox"
    image = "openwhisk/dockerskeleton"
    code  = filebase64("files/fetch_prices_rs.zip")
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
    code = filebase64("files/fetch_prices_js.zip")
  }

  limits {
    timeout = 10000
    memory  = 128
  }
}

