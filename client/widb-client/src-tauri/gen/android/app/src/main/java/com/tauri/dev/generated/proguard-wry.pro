# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!

# Copyright 2020-2023 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

-keep class com.tauri.dev.* {
  native <methods>;
}

-keep class com.tauri.dev.WryActivity {
  public <init>(...);

  void setWebView(com.tauri.dev.RustWebView);
  java.lang.Class getAppClass(...);
  java.lang.String getVersion();
}

-keep class com.tauri.dev.Ipc {
  public <init>(...);

  @android.webkit.JavascriptInterface public <methods>;
}

-keep class com.tauri.dev.RustWebView {
  public <init>(...);

  void loadUrlMainThread(...);
  void loadHTMLMainThread(...);
  void evalScript(...);
}

-keep class com.tauri.dev.RustWebChromeClient,com.tauri.dev.RustWebViewClient {
  public <init>(...);
}
