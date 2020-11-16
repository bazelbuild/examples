package exercise3.solution.logoreaderlarge;

import java.awt.image.BufferedImage;
import java.io.IOException;
import java.io.InputStream;
import javax.imageio.ImageIO;

/**
 * Starlark codelab main class.
 */
final class LogoReaderLarge {

  private LogoReaderLarge() {
  }

  public static void main(String[] args) {
    System.out.println("Running LogoReaderLarge");
    try {
      InputStream in = LogoReaderLarge.class.getResourceAsStream("logo_large.png");
      BufferedImage image = ImageIO.read(in);
      System.out.println("Logo Width = " + image.getWidth());
    } catch (IOException e) {
      System.out.println(e);
    }
  }
}
