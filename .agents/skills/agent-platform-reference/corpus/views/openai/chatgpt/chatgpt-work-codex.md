# Reading view: chatgpt-work-codex

> Generated from the normalized source mirror. Immutable response snapshots are retained under `corpus/snapshots/`; this view is disposable and may be regenerated.

- Source ID: `chatgpt-work-codex`
- Source URL: https://help.openai.com/en/articles/20001275-chatgpt-work-and-codex
- Resolved URL: https://help.openai.com/en/articles/20001275-chatgpt-work-and-codex
- Correction links: 1

## Curated corrections relevant to this source

### Correction 1

**Wrong: “One `SKILL.md` configures ChatGPT chat, Work, Codex, and Claude Code identically.”** Separate surfaces have distinct discovery, deployment, tool, and permission semantics. Sources: [ChatGPT Work and Codex](https://help.openai.com/en/articles/20001275-chatgpt-work-and-codex), [ChatGPT Skills](https://help.openai.com/en/articles/20001066-skills-in-chatgpt), [Agent Skills specification](https://agentskills.io/specification); local mirrors: `corpus/articles/openai/chatgpt/chatgpt-work-codex.md`, `chatgpt-skills.md`, `corpus/articles/standard/agent-skills/agent-skills-spec.md`.

[Open curated correction reference](../../../../references/corrections.md)

## Source mirror

<!-- source-id:chatgpt-work-codex -->

# ChatGPT Work and Codex

Use Chat or Work in ChatGPT, or switch to Codex for software development.

Updated: 2 days ago

# Overview

GPT‑6 Pro, powered by GPT‑6 Astra, is available in ChatGPT for Pro $100, Pro $200, Business and Enterprise plans. Enterprise access also depends on your workspace’s model-access permissions. Plus plans include GPT‑6 Astra in ChatGPT Work and Codex.

|  |
| --- |
| **Note**: Astra requires Codex CLI version **0.153.0 or newer**. Additionally, please update to the latest available ChatGPT Desktop app. If you do not see Astra available in Chat, Work or Codex in the desktop app, ensure that you update your app by clicking Menu > **Check for Updates,** even if you recently updated the app due to an automatic update notification or by clicking on the update icon in the sidebar. |

ChatGPT includes Chat and Work. Codex is available in the desktop app, and Codex Cloud lets you start and continue coding tasks from desktop, web, or mobile. Chat is for fast, conversational assistance and everyday questions. Work is an agent designed for longer, multi-step work and finished deliverables. Codex remains dedicated to software development and technical work.

[ChatGPT Voice](https://help.openai.com/articles/20001274) adds a live voice interface to Work on web and mobile, and to Work and Codex in the ChatGPT desktop app. You can speak, interrupt naturally, and use the tools and permissions available to the selected experience.

Work follows the same usage structure as Codex. The [Codex pricing page](https://developers.openai.com/codex/pricing) explains included usage and additional credits; its examples are based on coding tasks, so Work usage will vary by task.

# Models in Work and Codex

GPT-6.1 Sol, GPT-6 Sol and GPT-6 Luna are models for ChatGPT Work and Codex. They are not available in regular ChatGPT conversations. GPT-6.1 Sol is rolling out for eligible paid plans. Availability depends on your plan, workspace settings and rollout access.

Use the model picker to select Default or a specific model. Default combines models and reasoning levels along one slider. Selecting a model lets you choose from that model’s available reasoning levels. The options shown depend on your plan and workspace settings. Codex CLI does not use a slider.

Codex keeps the model you selected manually.

For workspace model access and usage details, see [ChatGPT Business models and limits](https://help.openai.com/articles/12003714) or [ChatGPT Enterprise and Edu - Models & Limits](https://help.openai.com/articles/11165333).

# Astra usage in Work and Codex

For Astra at Standard speed or in Fast mode, usage draws from your plan’s included Work and Codex allowance. Pro $100 and Pro $200 plans and Business Premium seats can use their full existing allowance for Astra. Plus and Business Standard seats include limited Astra usage, with optional credits for additional usage afterward.

Astra may use your allowance faster than GPT-5.6 Sol. The amount depends on the task, input and output size, reasoning level, and speed setting. Ultrafast has the plan and billing rules described below.

Signing in to Codex with ChatGPT uses your ChatGPT plan’s usage and billing. Using your own API key uses [API pricing](https://developers.openai.com/api/docs/pricing).

For usage paid in credits, see [personal-plan credit pricing](https://help.openai.com/articles/12642688) or the [Business, Enterprise and Edu credit rate card](https://help.openai.com/articles/11481834), as applicable. Enterprise customers billed in USD should use their [Enterprise USD rate card](https://help.openai.com/articles/20001415) and agreement.

# Ultrafast in Work and Codex

Ultrafast provides lower-latency GPT-6 Astra and GPT-6.1 Sol responses for eligible accounts.

At launch, Ultrafast is available on Pro $500 and in eligible Enterprise and Edu workspaces:

* **Pro $500**: Ultrafast uses your included Work and Codex usage first at 8× the Standard rate for the selected model. After that allowance is used, Ultrafast draws from your credit balance at 6× the Standard credit rate for the selected model.
* **Eligible Enterprise and Edu workspaces:** Ultrafast usage is charged to workspace credits. Access also depends on workspace permissions.

Ultrafast is not available at launch on Plus, Pro $100, Pro $200, or Business, including through credits.

When your account and workspace have access, select GPT-6 Astra or GPT-6.1 Sol, then choose **Ultrafast**.

For Pro plan details, see [About ChatGPT Pro tiers](https://help.openai.com/en/articles/9793128-about-chatgpt-pro-tiers). For credit-based usage, review the applicable [personal-plan credit pricing](https://help.openai.com/en/articles/12642688-using-credits-for-flexible-usage-in-chatgpt-freegopluspro-sora) or [Business, Enterprise and Edu credit rate card](https://help.openai.com/en/articles/11481834-chatgpt-rate-card-business-enterpriseedu-credit-based-pricing).

# Choose an experience

* **Chat:** Ask questions, search, brainstorm, or get quick conversational help.
* **Work:** Research a topic, analyze information, or create a document, spreadsheet, presentation, report, or Site.
* **Codex:** Write or debug code, run tests and commands, review changes, or work with a repository.
* **Voice**: Talk naturally with Work on web or mobile, or with Work or Codex in the ChatGPT desktop app, and ask it to work on a task.

# Where Work and Codex are available

Work is available on ChatGPT web and mobile for eligible paid plans. Work is also available in the ChatGPT desktop app when included for your plan and workspace.

Work is available for eligible ChatGPT Enterprise and Edu workspaces with Enterprise Key Management (EKM) enabled. For eligible workspaces, Work is enabled by default for members who use the default workspace role unless a workspace owner or admin turns it off, and Workspace settings and role-based access controls continue to determine each member’s access.

Work on web and mobile runs in the cloud. In the desktop app, select ChatGPT from the top-left menu, then select Work from the toggle at the top of the page. Chat and Work chats appear together in Recents, where you can sort, filter, and pin chats. Your existing ChatGPT Projects appear under Projects. From a project, choose Chat to start a new chat, or choose Work to start a Work chat using the project’s context. When allowed by your plan and workspace, Work in the desktop app can also use local files and desktop apps with your permission.

Cloud Work chats sync across web, mobile, and desktop. Work chats started on web or mobile appear in the desktop app, and cloud Work chats started on desktop can be continued on web or mobile. Local chats run on your computer. Messages and task context may be stored in the cloud, even when work runs locally.

Codex remains a separate view in the ChatGPT desktop app. Select **Codex** from the top-left menu. Its history remains separate from ChatGPT history. Codex can work with local folders, repositories, terminals, and developer tools.

Codex Cloud runs coding tasks on OpenAI-managed computers, so tasks can continue while your computer is asleep. Create and publish a cloud environment on desktop or web before selecting it for a mobile task. For plan availability and usage, see: [Using Codex with your ChatGPT plan](https://help.openai.com/articles/11369540).

You can access supported desktop Codex chats from the **Remote** tab in the ChatGPT mobile app. Remote access to a desktop task is separate from running a task in a cloud environment, and it does not add those desktop chats to your web or mobile chat history.

[ChatGPT Voice](https://help.openai.com/articles/20001274) is available in Work on web, iOS, and Android, and in Work and Codex in the ChatGPT desktop app on macOS and Windows.

# Use Voice with Work or Codex

## On web or mobile

Open ChatGPT and choose Work, then select the Voice control to start a Live conversation. Allow microphone access if prompted.

You can ask Work to use connected apps, create documents, presentations, and spreadsheets, or use a browser. Follow written responses and task results in the chat. If a task is still running when you end the call, Work can continue it in text.

## In the desktop app

* Open the ChatGPT desktop app and choose Work or Codex.
* Select the Voice control, or use a shortcut you configured in Settings.
* Allow microphone access when prompted. Computer context may also require Screen & Audio Recording and Accessibility permissions.
* Begin speaking. You can interrupt naturally, follow the live text in the chat, and ask Voice to start or coordinate tasks.

## Permissions and usage

Voice uses the tools and permissions available to the selected experience. In managed workspaces, Voice in Work requires both Voice and Work access. Plugins retain their existing app access, data permissions, and action restrictions.

On web and mobile, if an action needs your approval, review and approve or decline it using the on-screen controls. Spoken approval is not supported.

On web and mobile, Voice in Work uses minutes from your daily Voice allowance. Work tasks follow the same pricing whether you start them by speaking or typing.

In the desktop app, connected Voice time is metered separately where flexible pricing applies; tasks started through Voice draw from the same shared agentic usage and credit pool as Work and Codex. [Learn more about desktop Voice.](https://learn.chatgpt.com/docs/features/voice)

Only one Voice conversation can run at a time. See [ChatGPT Voice](https://help.openai.com/articles/20001274) for Voice settings, Chat Voice limits, retention, and data controls.

# Configure Work access for your workspace

Workspace owners and admins can manage Work for the default workspace role and custom roles in Workspace settings → **Permissions & roles.**

* **Work Cloud:** When enabled, members can start new cloud tasks and view existing cloud tasks across supported web, mobile, and desktop surfaces.
* **Work Local:** When enabled without Work Cloud, members can work locally in the desktop app but cannot start cloud tasks.
* **Codex Local:** Controlled separately, so changing Work Local or Work Cloud does not change access to Codex Local.
* **Browser use and network access:** Separate controls continue to apply where supported.

Chat remains available in every configuration. When Work or Codex Local is off for a member’s role, the member can see that the product is unavailable and request access from a workspace admin.

# Manage Codex Cloud access

Codex Cloud access is separate from Work Cloud and Codex Local. Managing workspace-shared environments requires additional permission. For plan availability, usage, and cloud-access requirements, see: [Using Codex with your ChatGPT plan](https://help.openai.com/articles/11369540).

If your workspace used the previous Codex cloud experience, Code Review, Security Review, and the existing Linear and GitHub integration workflows continue to use Codex Cloud (Legacy). For environment setup and the distinction between the two experiences, see: [Set up and configure Codex Cloud](https://developers.openai.com/codex/environments/cloud-environments).

# Set starting defaults for Work and Codex

Workspace owners and admins can use **Workspace settings > Models** to configure a starting model, reasoning level, speed, Fast Mode availability, and new-chat behavior for **Work & Codex**. Chat uses a separate starting default, so changing the Work & Codex setting does not change the starting model for a regular Chat conversation.

Where role-specific controls are available, role overrides are evaluated separately for Chat and Work & Codex. A starting default does not grant access to a model or feature that is unavailable to the member’s role.

See [**Managing workspace settings in ChatGPT Enterprise**](https://help.openai.com/articles/8411955) for the Chat default and role-inheritance behavior. Also review our article on [RBAC](https://help.openai.com/articles/11750701).

# Start on desktop

1. Open the ChatGPT desktop app and sign in.
2. Select ChatGPT or Codex from the top-left menu.
3. If you select ChatGPT, choose Chat or Work from the toggle at the top of the page.
4. Start a new chat or open an existing project. From a project, choose Chat to start a new chat, or choose Work to start a Work chat using project context. In Work or Codex, you can also open a local folder.
5. To use Voice in Work or Codex, select the Voice control, allow the required permissions, and begin speaking.
6. Describe the outcome you want. Add any files, constraints, and review criteria ChatGPT should use.
7. Review the result, then ask for changes or follow-up work in the same chat.

# Start Work on web or mobile

1. Open ChatGPT on the web or in the mobile app and select Work.
2. Start a new chat or open an existing project.
3. Add any files and context ChatGPT should use, then describe the task or deliverable you want completed

To work by speaking, select the Voice control in Work and start a Live conversation. You can review written responses and results in the chat.

A project keeps related chats, files, and instructions together. Work can also run once, repeat on a schedule or trigger, or monitor for changes through Scheduled Tasks. You can review progress, answer questions, change direction, and approve important actions.

Chats created in Chat sync between ChatGPT web and the desktop app. On desktop, Chat and Work chats appear together in Recents, where you can sort, filter, and pin chats.

Cloud Work chats sync across web, mobile, and desktop. Work chats started on web or mobile appear in the desktop app, and cloud Work chats started on desktop can be continued on web or mobile.

You can start and continue Codex Cloud tasks on web and mobile. To access supported desktop Codex chats, use the **Remote** tab in the ChatGPT mobile app. Remote access to a desktop task is separate from Codex Cloud.

# Create an event-triggered Work task

Eligible Plus, Pro, Business, Enterprise, Edu, and ChatGPT for Healthcare users can ask **Work** to run an event-triggered (webhook-based) task when a supported event occurs in a connected app. Supported events include new Gmail messages, new Slack channel messages, and GitHub pull request activity in an authorized repository.

Open the task to review **Trigger**, **Condition**, and **Prompt**. Go to **Scheduled** to manage it. Connected-app permissions, workspace controls, and approval requirements continue to apply.

For a Slack-triggered task, add **@ChatGPT** to every Slack channel the task monitors. Enterprise, Edu, and ChatGPT for Healthcare admins must enable **Allow event-triggered scheduled tasks** before members can create event-triggered tasks. In ChatGPT for Healthcare, these tasks are not covered under a Business Associate Agreement (BAA). Do not use them to transmit, store, or process protected health information (PHI).

Event-triggered tasks are not available on Free or Go, or in FedRAMP workspaces. Create or edit event triggers on the web or in supported mobile apps; the desktop app can display existing tasks but cannot create or edit their trigger conditions. Eligible users can share them with recipients who have access to **Work** and the required connected apps.

# Share a scheduled Work task

When scheduled task sharing is available for your account, you can share an eligible cloud-based scheduled task created in Work. An eligible recipient can review the shared task and schedule a separate copy. The copy can retain its Work mode and selected model when those options are available to the recipient.

The recipient needs their own access to Work, any required model, and any connected apps used by the task. Sharing does not transfer your chat history, local files, folders, device access, connected app credentials, saved memories, custom instructions, or workspace permissions. The task instructions are included in the shared link, so review them before sharing.

Event-triggered tasks can be shared and scheduled by eligible recipients. Tasks that run only on your computer cannot be scheduled from a shared link.

For more information, see: [Scheduled tasks in ChatGPT](https://help.openai.com/articles/10291617).

# FAQ

## Can Work use files on my computer?

Yes, in the desktop app. Open a local folder or project and grant access only to the files the task needs. Messages and task context may be stored in the cloud, even when work runs locally. Work on web and mobile cannot directly access files on your computer.

## Is Codex available on web or mobile?

Yes. With Codex Cloud, you can start and continue coding tasks from desktop, web, or mobile. Create and publish an environment on desktop or web before selecting it for a mobile task. Your plan and workspace’s cloud-access settings apply.

You can also access supported desktop Codex chats from the **Remote** tab in the ChatGPT mobile app. Remote access does not add those desktop chats to your web or mobile chat history.

## Where can I learn more?

For Codex access and usage, see: [Using Codex with your ChatGPT plan](https://help.openai.com/articles/11369540). For connecting repositories, preparing an environment, and configuring access to services, see: [Set up and configure Codex Cloud](https://developers.openai.com/codex/environments/cloud-environments). For organizing ongoing work, see: [Projects in ChatGPT](https://help.openai.com/articles/10169521). For recurring or trigger-based work, see: [Scheduled Tasks in ChatGPT](https://help.openai.com/articles/10291617).

## How do you switch between Chat, Work, and Codex?

**On desktop:**

Select ChatGPT or Codex from the top-left menu. In ChatGPT, select Chat or Work from the toggle at the top of the page. Chat and Work chats appear together in Recents. Use the filter to show all chats or only Chat or Work, and sort or pin chats as needed. Codex remains a separate view; its history remains separate from ChatGPT history.

While you’re in Codex, select New chat to open Quick Chat.

**On mobile:**

Select **Chat** or **Work** from the dropdown at the top of the screen. You can access supported desktop Codex chats from the **Remote** tab. You can also interact with Codex cloud tasks and projects on mobile.

## Can I use Voice with Work or Codex?

Yes. Eligible accounts can use Live in Work on web, iOS, and Android, and Voice in Work or Codex in the ChatGPT desktop app on macOS and Windows. Voice uses the tools and permissions available to the selected experience.

## What’s happening to regular ChatGPT?

The fast, familiar Chat experience is still available.

On desktop, select ChatGPT from the top-left menu, then select Chat from the toggle at the top of the page.

On mobile, select Chat from the dropdown at the top of the screen.

## Why can I use Work on web or mobile but not with local files in the desktop app?

As we roll out the Work feature, Cloud Work and local Work access may update at different times. First, ask a workspace owner or admin to confirm the Work permission assigned to your role. If the permission is enabled, allow time for the local-permission backfill to complete before contacting Support.
