//
//  AppDelegate.swift
//  LiveAppIcon
//
//  Created by 藤田大地 on 2023/05/28.
//

import Cocoa
import DSFDockTile
import DSFImageFlipbook
import ServiceManagement
import SwiftShell
import UserNotifications

@main
class AppDelegate: NSObject, NSApplicationDelegate {
    
    let liveAppIcon: DSFDockTile.Animated = {
        let fb = DSFImageFlipbook()
        let da = NSDataAsset(name: NSDataAsset.Name("LiveAppIcon"))!
        _ = fb.loadFrames(from: da.data)
        return DSFDockTile.Animated(fb)
    }()
    
    let command: String = {
        let asset = NSDataAsset(name: NSDataAsset.Name("Command"))!
        return String(data: asset.data, encoding: .utf8)!
            .trimmingCharacters(in: .whitespacesAndNewlines)
    }()
    
    let notificationCenter = UNUserNotificationCenter.current()
    
    func initialize() {
        Task {
            let setting = await notificationCenter.notificationSettings()
            if (setting.authorizationStatus == .notDetermined) {
                let _ = try? await notificationCenter.requestAuthorization(options: [.alert, .sound])
            }
            
            if (SMAppService.mainApp.status != .enabled) {
                try! SMAppService.mainApp.register()
            }
        }
    }
    
    func sendNotification(title: String, body: String) {
        let content = UNMutableNotificationContent()
        content.title = title
        content.body = body
        content.sound = UNNotificationSound.default
        let identifier = Bundle.main.bundleIdentifier!
        let request = UNNotificationRequest(
            identifier: identifier,content: content, trigger: nil)
        notificationCenter.getNotificationSettings { settings in
            if (settings.authorizationStatus == .authorized) {
                self.notificationCenter.add(request)
            }
        }
    }
    
    func applicationDidFinishLaunching(_ aNotification: Notification) {
        initialize()
        liveAppIcon.startAnimating()
    }
    
    func applicationWillTerminate(_ aNotification: Notification) {
        liveAppIcon.stopAnimating()
    }
    
    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        // Run the command when the app icon is clicked
        runCommand()
        return false
    }
    
    func runCommand() {
        Task {
            let result = SwiftShell.run(bash: command)
            if !result.succeeded {
                sendErrorNotification(error: result.stderror)
            }
        }
    }

    func sendErrorNotification(error: String) {
        let displayCommand = formatStringForNotification(message: command, limit: 16)
        let error = error.trimmingCharacters(in: .whitespacesAndNewlines)
        sendNotification(
            title: "Running '\(displayCommand)' failed",
            body: formatStringForNotification(message: error, limit: 96))
    }
    
    func formatStringForNotification(message: String, limit: Int) -> String {
        if (message.count > limit) {
            return "\(message.prefix(limit))..."
        } else {
            return message
        }
    }
}
