package homework01;

import java.io.*;
import java.nio.file.Paths;
import java.nio.file.Path;
import java.util.*;

import jFaaS.*;

import com.google.gson.JsonObject;

public class App {
  static final String URL_LONDON = "https://eu-gb.functions.appdomain.cloud/api/v1/web/62ea098b-618c-465f-8bed-aa659df82e70/default/nqueens";
  static final String URL_TOKYO = "https://jp-tok.functions.appdomain.cloud/api/v1/web/9e571da3-c444-42ab-93bc-8174744a38d5/default/nqueens";

  public int run(int k, int boardSize) {
    Map<String, Object> input = new HashMap();
    input.put("board_size", boardSize);

    var credentialsPath = Paths.get(".").toAbsolutePath().normalize().resolve("credentials.properties");
    var gateway = new Gateway(credentialsPath.toString());

    JsonObject londonResult = null;
    JsonObject tokyoResult = null;

    for (var i = 0; i < k; i++) {
      try {
        londonResult = gateway.invokeFunction(App.URL_LONDON, input);
        tokyoResult = gateway.invokeFunction(App.URL_TOKYO, input);
      } catch (IOException e) {
        e.printStackTrace();
        return 1;
      }
    }

    System.out.println("LONDON RESULT: " + londonResult.get("solutions"));
    System.out.println("TOKYO RESULT: " + tokyoResult.get("solutions"));

    return 0;
  }

  public static void main(String[] args) {
    var app = new App();

    System.exit(app.run(4, 8));
  }
}
