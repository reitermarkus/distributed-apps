package homework01;

import java.io.*;
import java.nio.file.Paths;
import java.nio.file.Path;
import java.util.*;

import jFaaS.*;

import com.google.gson.JsonObject;

public class App {
  Number nqueensMonolith(Gateway gateway, String url, Map<String, Object> input) throws IOException {
    JsonObject londonResult = gateway.invokeFunction(url, input);
    return londonResult.get("solutions").getAsNumber();
  }

  public int run(int k, int boardSize) {
    var url = System.getenv("NQUEENS_FUNCTION_URL");

    System.out.println("Running " + url + " with N=" + boardSize + " and k=" + k + " …");

    Map<String, Object> input = new HashMap();
    input.put("board_size", boardSize);

    var credentialsPath = Paths.get(".").toAbsolutePath().normalize().resolve("credentials.properties");
    var gateway = new Gateway(credentialsPath.toString());

    long start = System.nanoTime();

    for (var i = 0; i < k; i++) {
      try {
        System.out.println("Result " + i + ": " + nqueensMonolith(gateway, url, input));
      } catch (IOException e) {
        e.printStackTrace();
        return 1;
      }
    }

    long finish = System.nanoTime();
    long timeElapsed = finish - start;

    System.out.println("Took " + timeElapsed + " ns");

    return 0;
  }

  public static void main(String[] argv) {
    var app = new App();


    var args = new ArrayList<>(Arrays.asList(argv));

    var it = Arrays.stream(argv).iterator();

    var n = Optional.ofNullable(it.hasNext() ? it.next() : null).map(Integer::parseInt);
    if (n.isEmpty()) {
      System.err.println("Missing argument: n");
      System.exit(1);
    }
    var k = it.hasNext() ? Integer.parseInt(it.next()) : 1;


    System.exit(app.run(k, n.get()));
  }
}
