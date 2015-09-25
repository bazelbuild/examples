// Copyright 2015 The Bazel Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package com.google.bazel.example.android.activities;

import android.app.Activity;
import android.os.AsyncTask;
import android.os.Bundle;
import android.util.Log;
import android.view.Menu;
import android.view.MenuItem;
import android.widget.TextView;
import android.widget.Toast;

import org.json.JSONException;
import org.json.JSONObject;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import java.net.URLConnection;

public class MainActivity extends Activity {

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_main);
  }

  @Override
  public boolean onCreateOptionsMenu(Menu menu) {
    // Inflate the menu; this adds items to the action bar if it is present.
    getMenuInflater().inflate(R.menu.menu_main, menu);
    return true;
  }

  @Override
  public boolean onOptionsItemSelected(MenuItem item) {
    // Handle action bar item clicks here. The action bar will
    // automatically handle clicks on the Home/Up button, so long
    // as you specify a parent activity in AndroidManifest.xml.
    int id = item.getItemId();

    if (id == R.id.action_ping) {
      new AsyncTask<String, Void, String>() {
        public static final int READ_TIMEOUT_MS = 5000;
        public static final int CONNECTION_TIMEOUT_MS = 2000;

        private String inputStreamToString(InputStream stream) throws IOException {
          StringBuilder result = new StringBuilder();
          try {
            BufferedReader reader = new BufferedReader(new InputStreamReader(stream, "UTF-8"));
            String line;
            while ((line = reader.readLine()) != null) {
              result.append(line);
            }
          } finally {
            stream.close();
          }
          return result.toString();
        }

        private HttpURLConnection getConnection(String url) throws IOException {
          URLConnection urlConnection = new URL(url).openConnection();
          urlConnection.setConnectTimeout(CONNECTION_TIMEOUT_MS);
          urlConnection.setReadTimeout(READ_TIMEOUT_MS);
          return (HttpURLConnection) urlConnection;
        }

        @Override
        protected String doInBackground(String... params) {
          String url = params[0];
          HttpURLConnection connection = null;
          try {
            connection = getConnection(url);
            return new JSONObject(inputStreamToString(connection.getInputStream()))
                .getString("requested");
          } catch (IOException e) {
            Log.e("background", "IOException", e);
            return null;
          } catch (JSONException e) {
            Log.e("background", "JSONException", e);
            return null;
          } finally {
            if (connection != null) {
              connection.disconnect();
            }
          }
        }

        @Override
        protected void onPostExecute(String result) {
          TextView textView = (TextView) findViewById(R.id.text_view);
          if (result == null) {
            Toast.makeText(
                    MainActivity.this, getString(R.string.error_sending_request), Toast.LENGTH_LONG)
                .show();
            textView.setText("???");
            return;
          }
          textView.setText(result);
        }
      }.execute("http://10.0.2.2:8080/boop");
      return true;
    }

    return super.onOptionsItemSelected(item);
  }
}
