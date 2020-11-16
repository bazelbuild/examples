package exercise1.logoreadersmall;

import java.awt.image.BufferedImage;
import java.io.IOException;
import java.io.InputStream;
import javax.imageio.ImageIO;

/**
 * Starlark codelab main class.
 */
final class LogoReaderSmall {

  private LogoReaderSmall() {
  }

  public static void main(String[] args) {
    System.out.println("Running LogoReaderSmall");
    try {
      InputStream in = LogoReaderSmall.class.getResourceAsStream("logo_small.png");
      BufferedImage image = ImageIO.read(in);
      System.out.println("Logo Width = " + image.getWidth());
    } catch (IOException e) {
      System.out.println(e);
    }
  }
}
