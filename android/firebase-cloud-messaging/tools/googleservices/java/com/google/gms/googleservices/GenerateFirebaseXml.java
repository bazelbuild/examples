/*
 * Copyright (C) 2015 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Based on:
// https://android.googlesource.com/platform/tools/base/+/studio-3.0/build-system/google-services/src/main/groovy/com/google/gms/googleservices/GoogleServicesTask.java

package com.google.gms.googleservices;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import com.google.gson.JsonPrimitive;
import java.io.BufferedWriter;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Map;
import java.util.TreeMap;

/**
 * Creates Android resource XML for Google services (e.g. FCM) from a
 * google-services.json file.
 * 
 * Command line:
 * packageName inputJsonFile outputXmlFile
 */
public class GenerateFirebaseXml {

  private static final String STATUS_DISABLED = "1";
  private static final String STATUS_ENABLED = "2";
  private static final int WEB_CLIENT_TYPE = 3;

  public static void main(String[] args) throws Exception {
    
    if (args.length != 3) {
      System.err.println("3 arguments required:");
      System.err.println("packageName inputJsonFile outputXmlFile");
      System.exit(1);
    }
    
    String packageName = args[0];
    String inputJsonFile = args[1];
    String outputXmlFile = args[2];

    JsonElement root = new JsonParser().parse(Files.newBufferedReader(Paths.get(inputJsonFile)));

    if (!root.isJsonObject()) {
      throw new Exception("Malformed root json");
    }

    JsonObject rootObject = root.getAsJsonObject();

    Map<String, String> resValues = new TreeMap<>();
    Map<String, Map<String, String>> resAttributes = new TreeMap<>();

    handleProjectInfo(rootObject, resValues);

    JsonObject clientObject = getClientForPackageName(packageName, rootObject);

    if (clientObject != null) {
      handleDefaultWebClientId(clientObject, resValues);
      handleApiKey(clientObject, resValues);
      handleAdsService(clientObject, resValues);
      handleMapsService(clientObject, resValues);
      handleGoogleAppId(clientObject, resValues);
    } else {
      throw new RuntimeException("No matching client found for package name '" + packageName + "'");
    }

    try(BufferedWriter w = Files.newBufferedWriter(Paths.get(outputXmlFile))) {
      w.write(getValuesContent(resValues, resAttributes));
    }
  }

  private static void handleProjectInfo(JsonObject rootObject, Map<String, String> resValues)
      throws IOException {
    JsonObject projectInfo = rootObject.getAsJsonObject("project_info");
    if (projectInfo == null) {
      throw new RuntimeException("Missing project_info object");
    }

    JsonPrimitive projectNumber = projectInfo.getAsJsonPrimitive("project_number");
    if (projectNumber == null) {
      throw new RuntimeException("Missing project_info/project_number object");
    }

    resValues.put("gcm_defaultSenderId", projectNumber.getAsString());

    addProjectInfo(projectInfo, resValues, "project_id", "project_id");
    addProjectInfo(projectInfo, resValues, "firebase_url", "firebase_database_url");
    addProjectInfo(projectInfo, resValues, "storage_bucket", "google_storage_bucket");
  }

  private static void addProjectInfo(
      JsonObject projectInfo, Map<String, String> resValues, String sourceName, String destName)
  {
    if (projectInfo.has(sourceName)) {
      resValues.put(destName, projectInfo.getAsJsonPrimitive(sourceName).getAsString());
    }
  }

  private static void handleDefaultWebClientId(JsonObject clientObject,
      Map<String, String> resValues) {
    for (JsonElement oauthClientElement : clientObject.getAsJsonArray("oauth_client")) {
      JsonObject oauthClient = oauthClientElement.getAsJsonObject();
      int clientType = oauthClient.getAsJsonPrimitive("client_type").getAsInt();
      if (clientType == WEB_CLIENT_TYPE) {
        String clientId = oauthClient.getAsJsonPrimitive("client_id").getAsString();
        resValues.put("default_web_client_id", clientId);
        return;
      }
    }
  }

  private static void handleApiKey(JsonObject clientObject, Map<String, String> resValues) {
    if (clientObject.has("api_key")) {
      JsonArray apiKeys = clientObject.getAsJsonArray("api_key");
      JsonObject firstApiKey = apiKeys.get(0).getAsJsonObject();
      String currentKey = firstApiKey.getAsJsonPrimitive("current_key").getAsString();
      resValues.put("google_api_key", currentKey);
      resValues.put("google_crash_reporting_api_key", currentKey);
    }
  }

  /**
   * Handle a client object for analytics (@xml/global_tracker)
   * @param clientObject the client Json object.
   * @throws IOException
   */
  private static void handleAdsService(JsonObject clientObject, Map<String, String> resValues)
      throws IOException {
    JsonObject adsService = getServiceByName(clientObject, "ads_service");
    if (adsService == null) return;

    findStringByName(adsService, "test_banner_ad_unit_id", resValues);
    findStringByName(adsService, "test_interstitial_ad_unit_id", resValues);
  }

  private static void findStringByName(JsonObject jsonObject, String stringName,
      Map<String, String> resValues) {
    JsonPrimitive id = jsonObject.getAsJsonPrimitive(stringName);
    if (id != null) {
      resValues.put(stringName, id.getAsString());
    }
  }

  /**
   * Handle a client object for maps (@string/google_maps_key).
   * @param clientObject the client Json object.
   * @throws IOException
   */
  private static void handleMapsService(JsonObject clientObject, Map<String, String> resValues)
      throws IOException {
    JsonObject mapsService = getServiceByName(clientObject, "maps_service");
    if (mapsService == null) return;

    JsonArray array = clientObject.getAsJsonArray("api_key");
    if (array != null) {
      final int count = array.size();
      for (int i = 0 ; i < count ; i++) {
        JsonElement apiKeyElement = array.get(i);
        if (apiKeyElement == null || !apiKeyElement.isJsonObject()) {
          continue;
        }
        JsonObject apiKeyObject = apiKeyElement.getAsJsonObject();
        JsonPrimitive currentKey = apiKeyObject.getAsJsonPrimitive("current_key");
        if (currentKey == null) {
          continue;
        }
        resValues.put("google_maps_key", currentKey.getAsString());
        return;
      }
    }
    throw new RuntimeException("Missing api_key/current_key object");
  }

  /**
   * Handle a client object for Google App Id.
   */
  private static void handleGoogleAppId(JsonObject clientObject, Map<String, String> resValues)
      throws IOException {
    JsonObject clientInfo = clientObject.getAsJsonObject("client_info");
    if (clientInfo == null) {
      // Should not happen
      throw new RuntimeException("Client does not have client info");
    }

    JsonPrimitive googleAppId = clientInfo.getAsJsonPrimitive("mobilesdk_app_id");
    if (googleAppId == null) return;

    String googleAppIdStr = googleAppId.getAsString();
    if (googleAppIdStr == null || googleAppIdStr.isEmpty()) return;

    resValues.put("google_app_id", googleAppIdStr);
  }

  /**
   * find an item in the "client" array that match the package name of the app
   * @param jsonObject the root json object.
   * @return a JsonObject representing the client entry or null if no match is found.
   */
  private static JsonObject getClientForPackageName(String packageName, JsonObject jsonObject) {
    JsonArray array = jsonObject.getAsJsonArray("client");
    if (array != null) {
      final int count = array.size();
      for (int i = 0 ; i < count ; i++) {
        JsonElement clientElement = array.get(i);
        if (clientElement == null || !clientElement.isJsonObject()) {
          continue;
        }

        JsonObject clientObject = clientElement.getAsJsonObject();

        JsonObject clientInfo = clientObject.getAsJsonObject("client_info");
        if (clientInfo == null) continue;

        JsonObject androidClientInfo = clientInfo.getAsJsonObject("android_client_info");
        if (androidClientInfo == null) continue;

        JsonPrimitive clientPackageName = androidClientInfo.getAsJsonPrimitive("package_name");
        if (clientPackageName == null) continue;

        if (packageName.equals(clientPackageName.getAsString())) {
          return clientObject;
        }
      }
    }

    return null;
  }

  /**
   * Finds a service by name in the client object. Returns null if the service is not found
   * or if the service is disabled.
   *
   * @param clientObject the json object that represents the client.
   * @param serviceName the service name
   * @return the service if found.
   */
  private static JsonObject getServiceByName(JsonObject clientObject, String serviceName) {
    JsonObject services = clientObject.getAsJsonObject("services");
    if (services == null) return null;

    JsonObject service = services.getAsJsonObject(serviceName);
    if (service == null) return null;

    JsonPrimitive status = service.getAsJsonPrimitive("status");
    if (status == null) return null;

    String statusStr = status.getAsString();

    if (STATUS_DISABLED.equals(statusStr)) return null;
    if (!STATUS_ENABLED.equals(statusStr)) {
      System.err.println(String.format("Status with value '%1$s' for service '%2$s' is unknown",
          statusStr,
          serviceName));
      return null;
    }

    return service;
  }

  private static String getValuesContent(Map<String, String> values,
      Map<String, Map<String, String>> attributes) {
    StringBuilder sb = new StringBuilder(256);

    sb.append("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n" +
        "<resources>\n");

    for (Map.Entry<String, String> entry : values.entrySet()) {
      String name = entry.getKey();
      sb.append("    <string name=\"").append(name).append("\" translatable=\"false\"");
      if (attributes.containsKey(name)) {
        for (Map.Entry<String, String> attr : attributes.get(name).entrySet()) {
          sb.append(" ").append(attr.getKey()).append("=\"")
              .append(attr.getValue()).append("\"");
        }
      }
      sb.append(">").append(entry.getValue()).append("</string>\n");
    }

    sb.append("</resources>\n");

    return sb.toString();
  }
}
