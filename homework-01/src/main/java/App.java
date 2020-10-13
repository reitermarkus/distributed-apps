package homework01;

import java.io.*;
import java.nio.file.Paths;
import java.nio.file.Path;
import java.util.*;

import jFaaS.*;

import com.google.gson.JsonObject;

public class App {
  public int run(int k, int boardSize) {
    var url = System.getenv("NQUEENS_FUNCTION_URL");

    System.out.println("Running " + url + " with N=" + boardSize + " and k=" + k + " …");

    var input = new HashMap<String, Object>();
    input.put("board_size", boardSize);

    var credentialsPath = Paths.get(".").toAbsolutePath().normalize().resolve("credentials.properties");
    var gateway = new Gateway(credentialsPath.toString());

    long start = System.nanoTime();

    long resultTotal = 0;

    for (var i = 0; i < k; i++) {
      try {
        long resultStart = System.nanoTime();


        JsonObject londonResult = gateway.invokeFunction(url, input);
        var result = londonResult.get("solutions").getAsNumber();

        long resultEnd = System.nanoTime();
        long resultElapsed = resultEnd - resultStart;
        resultTotal += resultElapsed;

        System.out.println("Result " + i + ": " + result + " (" + resultElapsed + " ns)");
      } catch (IOException e) {
        e.printStackTrace();
        return 1;
      }
    }

    long finish = System.nanoTime();
    long timeElapsed = finish - start;

    System.out.println("Took " + timeElapsed + " ns (average " + resultTotal / k + " ns)");

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
