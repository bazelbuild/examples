package com.google.android.apps.common.testing.broker;

import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/** Converts modern SDK dexdump XML output into the XML shape expected by DexDumpIterator. */
final class ModernDexDumpOutputNormalizer {
  private ModernDexDumpOutputNormalizer() {}

  static void normalize(File dumpOutput) throws IOException {
    Files.write(
        dumpOutput.toPath(),
        normalize(Files.readAllLines(dumpOutput.toPath(), StandardCharsets.UTF_8)),
        StandardCharsets.UTF_8);
  }

  static List<String> normalize(List<String> lines) {
    List<String> output = new ArrayList<>();
    List<String> classAnnotations = new ArrayList<>();
    Map<String, List<String>> methodAnnotations = new HashMap<>();
    String annotationTarget = "";
    String annotationMethodName = "";
    String startElement = "";
    String startMethodName = "";

    for (String line : lines) {
      String trimmed = line.trim();

      if (trimmed.startsWith("Class #") && trimmed.endsWith(" annotations:")) {
        classAnnotations.clear();
        methodAnnotations.clear();
        annotationTarget = "";
        annotationMethodName = "";
        continue;
      }
      if ("Annotations on class".equals(trimmed)) {
        annotationTarget = "class";
        annotationMethodName = "";
        continue;
      }
      if (trimmed.startsWith("Annotations on method #")) {
        annotationTarget = "method";
        annotationMethodName = textBetweenLastQuotes(trimmed);
        continue;
      }
      if (trimmed.startsWith("Annotations on field #")) {
        annotationTarget = "";
        annotationMethodName = "";
        continue;
      }
      if (trimmed.startsWith("VISIBILITY_")) {
        String annotationClass = annotationClassName(trimmed);
        if (annotationClass != null) {
          if ("class".equals(annotationTarget)) {
            classAnnotations.add(annotationClass);
          } else if ("method".equals(annotationTarget) && !"".equals(annotationMethodName)) {
            methodAnnotations
                .computeIfAbsent(annotationMethodName, unused -> new ArrayList<>())
                .add(annotationClass);
          }
        }
        continue;
      }
      if ("empty-annotation-set".equals(trimmed)) {
        continue;
      }

      output.add(line);
      if (trimmed.startsWith("<class ")) {
        startElement = "class";
      } else if (trimmed.startsWith("<method ")) {
        startElement = "method";
        startMethodName = attributeValue(trimmed, "name");
      } else if (">".equals(trimmed)) {
        if ("class".equals(startElement)) {
          addAnnotations(output, classAnnotations);
        } else if ("method".equals(startElement)) {
          addAnnotations(output, methodAnnotations.get(startMethodName));
        }
        startElement = "";
        startMethodName = "";
      }
    }

    return output;
  }

  private static void addAnnotations(List<String> output, List<String> annotationClasses) {
    if (annotationClasses == null) {
      return;
    }
    for (String annotationClass : annotationClasses) {
      if (annotationClass.startsWith("dalvik.annotation.")) {
        continue;
      }
      output.add("<annotation type=\"" + annotationClass + "\">");
      output.add("</annotation>");
    }
  }

  private static String annotationClassName(String line) {
    int start = line.indexOf(" L");
    int end = line.indexOf(';', start);
    if (start == -1 || end == -1 || start >= end) {
      return null;
    }
    return line.substring(start + 2, end).replace('/', '.');
  }

  private static String attributeValue(String line, String attributeName) {
    String prefix = attributeName + "=\"";
    int start = line.indexOf(prefix);
    if (start == -1) {
      return "";
    }
    start += prefix.length();
    int end = line.indexOf('"', start);
    if (end == -1) {
      return "";
    }
    return line.substring(start, end);
  }

  private static String textBetweenLastQuotes(String line) {
    int end = line.lastIndexOf('\'');
    if (end == -1) {
      return "";
    }
    int start = line.lastIndexOf('\'', end - 1);
    if (start == -1) {
      return "";
    }
    return line.substring(start + 1, end);
  }
}
