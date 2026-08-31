package com.google.android.apps.common.testing.broker;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import com.google.android.apps.common.testing.proto.TestInfo.TestSuitePb;
import com.google.android.apps.common.testing.suite.dex.DumpUtils;
import java.io.ByteArrayInputStream;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.junit.runners.JUnit4;

@RunWith(JUnit4.class)
public class ModernDexDumpOutputNormalizerTest {
  @Test
  public void normalize_removesModernAnnotationPreambleAndAddsXmlAnnotations() {
    List<String> normalized =
        ModernDexDumpOutputNormalizer.normalize(
            modernDexDumpLines());

    assertEquals(
        Arrays.asList(
            "<api>",
            "<package name=\"com.example.android.instrumentation\"",
            ">",
            "<class name=\"GreeterInstrumentationTest\"",
            " extends=\"java.lang.Object\"",
            " interface=\"false\"",
            " abstract=\"false\"",
            " static=\"false\"",
            " final=\"true\"",
            " visibility=\"public\"",
            ">",
            "<annotation type=\"org.junit.runner.RunWith\">",
            "</annotation>",
            "<constructor name=\"GreeterInstrumentationTest\"",
            " type=\"com.example.android.instrumentation.GreeterInstrumentationTest\"",
            " static=\"false\"",
            " final=\"false\"",
            " visibility=\"public\"",
            ">",
            "</constructor>",
            "<method name=\"testDefaultGreeting\"",
            " return=\"void\"",
            " abstract=\"false\"",
            " native=\"false\"",
            " synchronized=\"false\"",
            " static=\"false\"",
            " final=\"false\"",
            " visibility=\"public\"",
            ">",
            "<annotation type=\"org.junit.Test\">",
            "</annotation>",
            "</method>",
            "<method name=\"testNamedGreeting\"",
            " return=\"void\"",
            " abstract=\"false\"",
            " native=\"false\"",
            " synchronized=\"false\"",
            " static=\"false\"",
            " final=\"false\"",
            " visibility=\"public\"",
            ">",
            "<annotation type=\"org.junit.Test\">",
            "</annotation>",
            "</method>",
            "</class>",
            "<class name=\"Helper\"",
            " extends=\"java.lang.Object\"",
            " interface=\"false\"",
            " abstract=\"false\"",
            " static=\"false\"",
            " final=\"true\"",
            " visibility=\"public\"",
            ">",
            "</class>",
            "",
            "</package>",
            "</api>"),
        normalized);
  }

  @Test
  public void normalize_outputCanBeParsedForJUnit4Tests() {
    String normalizedXml = String.join("\n", ModernDexDumpOutputNormalizer.normalize(
        modernDexDumpLines()));

    TestSuitePb suite =
        DumpUtils.parseDexDump(
                new ByteArrayInputStream(normalizedXml.getBytes(StandardCharsets.UTF_8)))
            .getFirst();

    assertEquals(2, suite.getInfoCount());
    assertEquals("GreeterInstrumentationTest", suite.getInfo(0).getTestClass());
    assertEquals("com.example.android.instrumentation", suite.getInfo(0).getTestPackage());

    List<String> methods = new ArrayList<>();
    for (int i = 0; i < suite.getInfoCount(); i++) {
      methods.add(suite.getInfo(i).getTestMethod());
    }

    assertTrue(methods.contains("testDefaultGreeting"));
    assertTrue(methods.contains("testNamedGreeting"));
  }

  private static List<String> modernDexDumpLines() {
    return Arrays.asList(
        "<api>",
        "Class #7 annotations:",
        "Annotations on class",
        "  VISIBILITY_RUNTIME Lorg/junit/runner/RunWith; value=Landroidx/test/runner/AndroidJUnit4;",
        "Annotations on method #10 'testDefaultGreeting'",
        "  VISIBILITY_RUNTIME Lorg/junit/Test;",
        "Annotations on method #11 'testNamedGreeting'",
        "  VISIBILITY_RUNTIME Lorg/junit/Test;",
        "Annotations on method #11 '<init>'",
        "  VISIBILITY_SYSTEM Ldalvik/annotation/Signature; value={ ()V }",
        "<package name=\"com.example.android.instrumentation\"",
        ">",
        "<class name=\"GreeterInstrumentationTest\"",
        " extends=\"java.lang.Object\"",
        " interface=\"false\"",
        " abstract=\"false\"",
        " static=\"false\"",
        " final=\"true\"",
        " visibility=\"public\"",
        ">",
        "<constructor name=\"GreeterInstrumentationTest\"",
        " type=\"com.example.android.instrumentation.GreeterInstrumentationTest\"",
        " static=\"false\"",
        " final=\"false\"",
        " visibility=\"public\"",
        ">",
        "</constructor>",
        "<method name=\"testDefaultGreeting\"",
        " return=\"void\"",
        " abstract=\"false\"",
        " native=\"false\"",
        " synchronized=\"false\"",
        " static=\"false\"",
        " final=\"false\"",
        " visibility=\"public\"",
        ">",
        "</method>",
        "<method name=\"testNamedGreeting\"",
        " return=\"void\"",
        " abstract=\"false\"",
        " native=\"false\"",
        " synchronized=\"false\"",
        " static=\"false\"",
        " final=\"false\"",
        " visibility=\"public\"",
        ">",
        "</method>",
        "</class>",
        "Class #8 annotations:",
        "Annotations on class",
        "  VISIBILITY_SYSTEM Ldalvik/annotation/AnnotationDefault;",
        "<class name=\"Helper\"",
        " extends=\"java.lang.Object\"",
        " interface=\"false\"",
        " abstract=\"false\"",
        " static=\"false\"",
        " final=\"true\"",
        " visibility=\"public\"",
        ">",
        "</class>",
        "",
        "</package>",
        "</api>");
  }
}
