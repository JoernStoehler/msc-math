# Plugins in ChatGPT

Learn how plugins work in ChatGPT Chat, Work and Codex, how to install them, and how connected apps and workspace settings affect access.

Updated: 2 days ago

# Overview

Plugins expand what you can do with ChatGPT and Codex by bringing together the tools, knowledge, and reusable workflows you need to get work done.  
  
A plugin can include:

* [Skills](https://help.openai.com/articles/20001066) that provide instructions and workflow guidance.
* [Connected apps](https://help.openai.com/articles/11487775) that link external accounts, information, and actions.
* [App templates](https://help.openai.com/articles/20001247) that help administrators configure apps for a workspace.
* Extensions that let you interact with apps, mention items in a prompt, open or edit supported files, and use forms or plugin settings.

Some plugins include multiple apps. Others include only skills and do not require an app connection. An app connects ChatGPT to a service, such as Slack. A plugin brings apps and skills together for a task, and may require authentication with your account on that service. [Read more about apps](https://help.openai.com/articles/11487775).

Installing a plugin may start an app-connection or setup flow. You must still complete any required account authorization, and installation cannot bypass provider or workspace permissions.

## Availability

The plugin directory is available across ChatGPT plans. Installing or using an individual plugin depends on your plan, workspace, role, region, and its included capabilities.

# Find plugins

Go to **Plugins** on the sidebar on ChatGPT web, desktop or mobile to browse for and find plugins for your use case. The directory shows plugins available to your plan and workspace - click on a plugin and review its description, skills, apps, and setup requirements.

A plugin’s availability and features can vary depending on where you use it. Some tools, interactive views, or apps may be available only on desktop, web, or mobile.

Browse categories such as **Popular** and **New & Noteworthy**, or review a plugin suggested during a conversation. The options you see depend on your account and where you use ChatGPT.

* **Public**: Browse publicly available plugins.
* **Your workspace**: Find plugins available in your workspace’s directory.
* **Personal**: Find plugins you created or that others shared with you.
* **Installed**: Return to plugins you’ve installed.

## OpenAI Verified plugins

Some plugins display an **OpenAI Verified** badge. OpenAI reviews these plugins for quality, reliability, and usefulness.  
  
Go to [Admin console](https://admin.openai.com), select the intended ChatGPT workspace, and open **Plugins**. Workspace owners and administrators can review verification status there.  
  
A verification badge does not replace your organization's privacy, security, or vendor review.

# Install a plugin and connect its apps

Before installing a plugin, review its included apps and setup requirements. A **Desktop only** label means the plugin requires the desktop app.

1. Go to the [plugin directory](https://chatgpt.com/plugins) in ChatGPT.
2. Select the plugin and review its included apps.
3. Select **Install plugin** if shown; installation may start the app’s setup or authorization flow.
4. If prompted, select **Connect** for an app that requires account authorization.
5. Sign in to the intended provider account only when individual authorization is required.
6. If an app supports sync, ask an eligible workspace administrator to configure it.

Some apps ask you to sign in to your own account. Others need no sign-in or use a connection set up by your workspace administrator.

For account selection, reconnecting, multiple accounts, and disconnection, see: [Connecting and managing app accounts in ChatGPT](https://help.openai.com/articles/20001494).

# Use a plugin

ChatGPT automatically uses your installed plugins when they’re relevant to your request. You can also select a plugin directly with an @ mention, select **+** and enter its name, or select **Plugins** in the composer.

In a supported Codex task view:

1. Open **Sources**.
2. Select **Use plugins**.
3. Search for and select an installed plugin.

Available options depend on where you use ChatGPT, your plan, and your workspace settings.

## Automate a task

You can ask ChatGPT to automate a task, such as preparing a daily summary or checking for updates every hour. Some plugins also support events, which let ChatGPT respond when something happens, such as a document changing or someone replying in a thread. Tell ChatGPT what to watch for and what you’d like it to do.

Note that not all plugins will support events. More plugins will support events as plugin developers add the functionality to their plugins. Learn more about [submitting a plugin to the plugin directory](https://developers.openai.com/plugins/deploy/submission).

Review the event, conditions, and task instructions, and connect any required app. For task setup and management, see: [Scheduled tasks in ChatGPT](https://help.openai.com/articles/10291617).

## Use plugin extensions

Some plugins include interactive apps you can open in full screen by selecting their icon in the ChatGPT sidebar. Browse and interact with the app, then ask ChatGPT to help with what you’re working on.

Plugins can also add interactive views within a conversation, let you mention specific items in your prompt, or open and edit supported files. Some include their own settings or forms to collect information needed for a task.

Available features depend on the plugin and where you use ChatGPT. You’ll still need to connect any required accounts and have the appropriate workspace permissions.

Site tools are a separate feature that lets ChatGPT use actions provided by an open webpage. For details, see: [Using site tools in the ChatGPT desktop app](https://help.openai.com/articles/20001423).

# Create plugins

Create a plugin to bring the instructions and tools for a task together in one place. You can include skills, connected apps, or both.

To get started, install Plugin Creator from the plugin directory. Then start a chat with @plugin-creator and describe what you want to build.

For example, you could ask:

“Create a Project Updates plugin that gathers progress from Slack and Linear and drafts a weekly update with key milestones, blockers, and next steps. Add a skill that uses the attached template to keep updates consistent.”

Work with ChatGPT to create your plugin. Local plugins install automatically and may not display an install card. Install other plugins if prompted. Complete any required app connections.

![Finance Team Handbook plugin card with its description and Not now and Install buttons.](https://images.ctfassets.net/j22is2dtoxu1/plugins_img_099048c330d44aa985aa1ac0c02df953/3bfa18e686e72533f19d8102af325e36/plugin-installation-card.png?q=80&fm=webp&w=1344)

Try your plugin by asking it to draft a weekly project update. Check that it reflects the activity in Slack and Linear and follows your template. If your plugin can make changes, test those actions and review the results before sharing it.

You can use your plugin by mentioning it with @. For other options, see the Use a plugin section above.

You can also start from the plugin directory: go to **Plugins** and select **Add (+)** to open a conversation with Plugin Creator. Install Plugin Creator first if prompted.

## Choose what your plugin contains

Choose the capabilities your plugin needs. Availability depends on where you use ChatGPT or Codex:

* A **skill** can provide reusable instructions, such as a weekly-update format, without connecting to an external service.
* Your plugin can include **local MCP apps** that run on your computer, and you can run the plugin on ChatGPT Desktop. Note that saving a plugin with local apps to your account does not make those tools available on web or mobile.
* Include an app created for your workspace to connect to external services. For setup instructions, see: [Creating MCP apps](https://help.openai.com/articles/12584461).
* Use ChatGPT Sites to host an MCP app and include that app in your plugin. Recipients also need access to the Site. For setup instructions, see: [Hosting a plugin with ChatGPT Sites](https://help.openai.com/en/articles/20001547).
* Add interactive apps or views where plugin extensions are supported.

You can create a plugin with an app hosted on ChatGPT Sites on any ChatGPT plan. In a managed workspace, your administrator must enable the required Sites and plugin permissions.

Users on Business and Enterprise workspaces can share plugins with other members of their workspace. Read more below: Share and publish plugins in the workspace.

## Edit a plugin

You can edit a workspace plugin if you're its creator or a workspace owner or administrator with the required permissions. Access to a shared plugin doesn't grant permission to edit it.

To update your plugin, mention @plugin-creator and the existing plugin, then describe what you’d like to change. For example: “@plugin-creator Update Project Updates to include a summary of upcoming deadlines.” Review and test the updated plugin before sharing it.

# Upload or update a plugin ZIP file

If you have a plugin ZIP file, you can upload it in ChatGPT on the web when the upload option is available. You need upload permission or access as an eligible workspace owner or administrator.

## Upload a plugin

If you're a workspace owner or administrator with upload access:

1. Go to [Admin console](https://admin.openai.com), select the intended ChatGPT workspace, and open **Plugins**.
2. Select **Add**.
3. Select **Upload plugin** to upload your ZIP file.

## Update a manually uploaded plugin

You can upload a new version of a manually uploaded plugin in your current workspace if you have the required creator, editor, or administrator permissions.

1. Open the plugin's details.
2. Select **Upload new version** when available.

For a plugin managed from a source, update it through that source. For GitHub marketplace updates, see: [Importing and syncing plugin marketplaces from GitHub](https://help.openai.com/articles/20001504).

Uploading a plugin doesn't grant access to its included apps or complete their authorization. Complete any required app authorization before use. Plugins marked **Desktop only** can't run in ChatGPT on the web.

# Understand how plugins use apps

A plugin can use a connected app to search information, retrieve content, or complete supported actions.  
  
Apps used by a plugin remain subject to their own controls, including:

* Workspace availability and applicable role access.
* Provider-account authorization.
* Supported read and write actions.
* Action approval requirements.
* Approved account domains.
* Sync and source restrictions, when supported.

A plugin cannot use an app to access content outside the permissions granted to the relevant individual provider account or administrator-managed source.  
  
If an optional app is disabled, other plugin capabilities may continue to work. If a required app is unavailable, the related capability cannot run.  
  
For app approval settings, see: [Managing app permissions in ChatGPT](https://help.openai.com/articles/20001495).  
  
For information about data shared with apps and how it is used, see: [Connected apps in ChatGPT](https://help.openai.com/articles/11487775).

## Plugins that include app templates

An app template is not a ready-to-use app. An administrator may need to configure the provider connection, publish a workspace-specific app, and assign access before members can use it.  
  
Installing the plugin does not complete those steps.  
  
For setup instructions, see: [ChatGPT app templates](https://help.openai.com/articles/20001247).

## Plugins that use administrator-managed sync

Some supported apps can index selected workspace content after an eligible administrator configures sync.  
  
Installing a plugin does not create a sync connection. Individually authorized sync is unavailable; supported sync must be configured by a workspace administrator.  
  
For eligibility, indexing, data residency, and controls, see: [Administrator-managed apps with sync in ChatGPT](https://help.openai.com/articles/10847137).

# Manage plugins in a workspace

Go to [Admin console](https://admin.openai.com), select the intended ChatGPT workspace, and open **Plugins**. Workspace administrators can manage plugins and their included apps there.  
  
Plugin and app controls remain separate even when they appear on the same plugin detail page. Plugin settings govern availability and installation; app settings govern account access, actions, sync, and provider restrictions.  
  
In Business workspaces, apps are enabled by default. New Enterprise and Edu workspaces start with a selected set of apps enabled; these defaults do not change existing workspace settings and do not apply to Healthcare workspaces. Availability depends on the plugin, its required apps, workspace policy, and applicable role settings.  
  
Enterprise and Edu administrators can configure plugin access by custom role. A role can use its own setting or inherit the workspace default. Access to an included app is evaluated separately. Business administrators manage workspace-wide availability.  
  
For workspace navigation and app access, see: [Admin controls, security, and compliance for plugins and apps](https://help.openai.com/articles/11509118).

## Review plugin permissions

In a managed workspace, review the controls available to your workspace under **Workspace settings > Permissions & roles**. Select a role to review its permissions.

* **Use plugins**: Allows members to use plugins. On by default for Enterprise.
* **Upload plugins**: Allows members to upload plugins. Off by default for Enterprise.
* **Create plugins with MCPs**: Allows members to create plugins that use MCP servers. Off by default for Enterprise.
* **Share plugins**: Allows sharing with specific people or groups, or anyone in the workspace with the link.
* **Publish plugins to workspace**: Allows publishing a plugin to the workspace directory.

Creating a plugin with an app hosted on ChatGPT Sites requires **Use plugins**, **Upload plugins**, and **Create plugins with MCPs**. The last permission also allows a Site to connect to an external MCP server. Recipients need access to the underlying Site separately.

## Set an installation policy

Installation policies may include:

* **Available**: Eligible members can install the plugin.
* **Installed**: The plugin is installed automatically for eligible workspace members or, where role-based controls are available, the selected role.

Neither installation policy overrides provider permissions or grants access to data the relevant connection cannot access.  
  
To configure a plugin:

1. Go to **Workspace settings** > **Plugins**.
2. Select the plugin.
3. Review its skills, required apps, optional apps, and app templates.
4. Select an available installation policy.
5. Confirm any required app is enabled and accessible.
6. On the plugin detail page, review the included app's access, allowed actions, and available sync settings.
7. Ask an eligible member to test the workflow. Connect an individual provider account only if the requested live app feature requires one.

For detailed workspace controls, see: [Admin controls, security, and compliance for plugins and apps](https://help.openai.com/articles/11509118).

## Import and sync a marketplace from GitHub

Workspace admins can import a plugin marketplace from a public or private GitHub repository and keep its plugins up to date with daily sync. A marketplace is a JSON catalog that lists the plugins to import. For setup, access requirements, and troubleshooting, see: [Importing and syncing plugin marketplaces from GitHub](https://help.openai.com/articles/20001504).  
  
Use the update control that matches what you want to refresh:

* **Synced GitHub marketplace:** Go to [Admin console](https://admin.openai.com), select the intended ChatGPT workspace, and open **Plugins**. Open **Marketplaces** and select the marketplace. Select **Sync now** to request an update from GitHub.
* **Individually imported plugin:** If its admin view shows **Refresh**, use that action to refresh the plugin.
* **Displayed list:** **Refresh plugin list** reloads the list. It does not sync from GitHub.

# Share and publish workspace plugins

New workspace plugins start private. Review and test your plugin before sharing it. Recipients need to install the plugin before using it. Sharing doesn't change access or authorization requirements for its included apps.

If your plugin includes an app hosted on ChatGPT Sites, recipients also need viewer access to the Site and must connect the app themselves. Sharing a plugin and publishing it to the workspace directory require separate permissions.

Workspace administrators control whether members can share plugins or publish them to a workspace directory.

To review sharing and publishing permissions, go to **Workspace settings** > **Permissions & roles** and select the role. Check **Share plugins** and **Publish plugins to workspace**.

Enabling a permission does not publish a plugin automatically.

## Share a plugin you own

1. Go to **Plugins** and select a plugin you own.
2. Open the more options menu (•••).
3. Select **Share plugin**.
4. Under **Who has access**, select an available option.
5. Select **Save**.

Access options may include:

* **Only those invited**: Only specified people or groups can access the plugin.
* **Anyone in this workspace with the link**: Workspace members with the link can access the plugin.
* **Visible in workspace directory**: Eligible members can find and install the plugin.

The exact workspace name appears in the applicable option.  
  
Publishing to a workspace directory does not publish the plugin to the public directory. Sharing does not change the underlying apps' permissions.

# Disable a plugin

Plugin installation, app access, and sync are separate controls.

1. Go to **Workspace settings** > **Plugins**.
2. Select the plugin and open its more options menu (•••).
3. Select **Disable** or **Disable plugin**, depending on the label shown.
4. Review affected apps and any warning before confirming. If prompted, select **Disable plugin and app** only after checking the impact on shared apps and sync.

A shared app may be used by more than one plugin. Disabling it can affect other workflows and workspace members.  
  
Disabling an administrator-managed synced source or deleting its connection can permanently remove indexed data. Review the warning before confirming because the change may affect other authorized users or plugins.  
  
Disabling an app does not necessarily uninstall a plugin. The plugin may remain visible in Codex, and skills that do not depend on the app may continue to work.  
  
For workspace controls, see: [Admin controls, security, and compliance for plugins and apps](https://help.openai.com/articles/11509118).

# FAQ

## What is the difference between an app and a plugin?

An app connects ChatGPT to an external service, such as Google Drive or Slack, so it can access information or perform supported actions. A plugin packages capabilities into a workflow and can include one or more apps, skills, or both. Some plugins include only skills and do not connect to an external service. Installing a plugin does not bypass an app’s authorization requirements or workspace permissions. For details, see: [Connected apps in ChatGPT](https://help.openai.com/articles/11487775).

## How can I export the public plugin catalog?

Go to [Admin console](https://admin.openai.com), select the intended ChatGPT workspace, and open **Plugins**. Eligible workspace owners and administrators can select **Export CSV**.  
  
The export can include plugin, app, and skill details, developers, versions, dates added, and verification status. It does not include plugins created in your workspace.  
  
The export uses a daily snapshot, may be up to 48 hours old, and is not available in FedRAMP workspaces.

## Why can't I find a plugin?

Check the plugin’s plan, region, and workspace requirements, and whether it works where you’re using ChatGPT or Codex.  
  
In a managed workspace, ask an administrator to review workspace availability, the installation policy, required apps, and, in Enterprise and Edu, your applicable role settings. A restricted plugin can remain visible in the directory with **Disabled by admin**. If you use a personal account without an administrator, check the plan and account requirements or contact OpenAI Support.  
  
In Codex, directory changes can take up to 6 hours to refresh.  
  
For app-specific problems, see: [Troubleshooting apps in ChatGPT](https://help.openai.com/articles/20001497).

## Why does a plugin say an app needs setup?

The app may need administrator configuration, provider authorization, or setup from an app template.  
  
For account connection steps, see: [Connecting and managing app accounts in ChatGPT](https://help.openai.com/articles/20001494).

## Why can't a plugin access information or complete an action?

Check that:

* The underlying app is available.
* The relevant individual account or administrator-managed source can access the information.
* The app supports the action.
* Applicable workspace settings permit the action.

For approval settings, see: [Managing app permissions in ChatGPT](https://help.openai.com/articles/20001495).  
  
For additional diagnosis, see: [Troubleshooting apps in ChatGPT](https://help.openai.com/articles/20001497).

## Why is a plugin still visible after its app was disabled?

Disabling an app blocks capabilities that require that app. It does not necessarily uninstall the plugin or remove skills that remain available independently.

## Why can I use a plugin only in Codex?

Some plugins are local to Codex or designed for Codex-specific workflows. An administrator may need to make the plugin available before it can be used elsewhere.

## Why is a plugin marked Desktop only?

A plugin marked **Desktop only** requires the desktop app and can’t run in ChatGPT on the web. Some imported plugins receive this label because of how their tools are configured. Ask the plugin author or your workspace administrator about a version that supports web use.

## Why is a plugin view or feature missing?

The plugin may not include that feature, or it may not support it where you’re using ChatGPT. Check the plugin’s description and requirements, and confirm that any required app is available and connected. In a managed workspace, ask an administrator to check your access.

A plugin’s extension support is separate from the restrictions described under **Desktop only**. Extension availability does not remove an existing desktop restriction.

For event setup, see: [Scheduled tasks in ChatGPT](https://help.openai.com/articles/10291617). For tools provided by a webpage, see: [Using site tools in the ChatGPT desktop app](https://help.openai.com/articles/20001423).

## Why don't I have access to skills?

Personal Skills are generally available to ChatGPT Business, Enterprise, Healthcare, and Edu users. Availability also depends on workspace settings, role, and product surface. Skills in ChatGPT and Codex may need to be managed separately.

## How does a plugin become OpenAI Verified?

OpenAI works with selected developers to review plugins for quality, reliability, and usefulness. The **OpenAI Verified** badge identifies plugins included in that program.
