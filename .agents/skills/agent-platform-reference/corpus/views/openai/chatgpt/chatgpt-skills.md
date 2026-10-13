# Reading view: chatgpt-skills

> Generated from the normalized source mirror. Immutable response snapshots are retained under `corpus/snapshots/`; this view is disposable and may be regenerated.

- Source ID: `chatgpt-skills`
- Source URL: https://help.openai.com/en/articles/20001066-skills-in-chatgpt
- Resolved URL: https://help.openai.com/en/articles/20001066-skills-in-chatgpt
- Correction links: 1

## Curated corrections relevant to this source

### Correction 1

**Wrong: “One `SKILL.md` configures ChatGPT chat, Work, Codex, and Claude Code identically.”** Separate surfaces have distinct discovery, deployment, tool, and permission semantics. Sources: [ChatGPT Work and Codex](https://help.openai.com/en/articles/20001275-chatgpt-work-and-codex), [ChatGPT Skills](https://help.openai.com/en/articles/20001066-skills-in-chatgpt), [Agent Skills specification](https://agentskills.io/specification); local mirrors: `corpus/articles/openai/chatgpt/chatgpt-work-codex.md`, `chatgpt-skills.md`, `corpus/articles/standard/agent-skills/agent-skills-spec.md`.

[Open curated correction reference](../../../../references/corrections.md)

## Source mirror

<!-- source-id:chatgpt-skills -->

# Skills in ChatGPT

Learn how skills work in ChatGPT, how to create and share them, and how workspace administrators manage access.

Updated: last month

# Overview

Skills are reusable, shareable workflows that help ChatGPT complete specific tasks more consistently. A skill can include instructions, examples, and code. After a skill is created and installed, ChatGPT can automatically use one or more skills when they are helpful.  
  
A skill can include instructions and supporting resources you want ChatGPT to use for a specific task. It can also include reusable steps and scripts for more structured work.  
  
Skills are available to eligible ChatGPT Business, Enterprise, Healthcare, and Edu users, subject to workspace settings and product availability. Skills can also be supported in Codex and other OpenAI products. Availability, installation, and syncing can differ by product and surface.  
  
A plugin can include skills, apps, or both. Skills provide reusable instructions, while apps connect to external services. For plugin details, see: [Plugins in ChatGPT and Codex](https://help.openai.com/articles/20001256).

# Access and create skills

To find your skills in ChatGPT:

1. In the sidebar, select **Plugins**.
2. In the **Plugin Directory**, select the **Skills** tab.

On the **Skills** page, you can see skills that are **Installed**, **Created by me**, **Shared with me**, and **Shared by {workspace name}**. Eligible accounts include one skill by default: skill-creator. When you ask ChatGPT to create or modify a skill, it automatically uses the skill-creator skill to help generate, update, or troubleshoot it.  
  
You can create or install a skill in any of the following ways:

* **Create with chat**: Go to **Skills**, select **Create**, then select **Create with chat**. You can also ask ChatGPT to create a skill directly in your chat. ChatGPT will ask follow-up questions and prompt you to install the skill.
* Install skills shared with you or your workspace: Go to **Skills** > **Shared with me** or **Shared by {workspace name}**, then select the more options menu (•••) for the skill and select **Install**.
* **Create with editor**: Build and manage skills in the [Skills editor](https://chatgpt.com/skills/editor). Go to **Skills**, select **Create**, then select **Create with editor**.
* Upload a skill: Go to **Skills**, select **Create**, then select **Upload from your computer**.

# Share skills with your workspace

You can share skills with teammates or publish them to your workspace library so others can use the same workflow.  
  
You can share a skill with your workspace from **Skills**:

1. In the sidebar, select **Plugins**.
2. In the **Plugin Directory**, select the **Skills** tab.
3. Select the more options menu (•••) for the skill you want to share.
4. Select **Share**.

You can search for people or groups in your workspace to share the skill, or copy the sharing link directly. You can also set access permissions so that only you or specific people have access to the skill in your workspace.

# Uploaded skills

You can upload a skill by selecting **Create** and then **Upload from your computer**. Before uploading a skill, especially one downloaded from an external source or shared by another organization, review it and make sure you trust its source. Skills can include instructions, supporting files, and code.  
  
When you upload a skill, ChatGPT scans it before it becomes available. Most uploaded skills are available immediately after the scan completes. Some skills may be marked **Needs Review**, requiring you to review additional information before using them. Skills that appear to contain content or behavior that could pose a risk may be marked **Blocked** and cannot be used. The scan should not replace your own review, policies, or judgment when determining whether an uploaded skill is appropriate to use.

# Admin controls for Enterprise and Edu

Skill availability and workspace defaults depend on the current product configuration and administrator settings. [Enterprise](https://chatgpt.com/business/enterprise/) and Edu workspace administrators can review which roles are allowed to create, use, share, or install skills.  
  
Workspace admins can set the following [**Permissions & roles**](https://chatgpt.com/admin/permissions) for skills:

* **Enable skills**: Allow members to create and use skills.
* **Enable skill uploading**: Allow members to upload skill files from their computer to the workspace.
* **Share skills**: Allow members to share skills with workspace members, groups, or the whole workspace.
* **Publish skills to workspace**: Allow members to publish and share skills with the entire workspace.
* **Enable skills installing**: Allow members to install skills for other workspace members so they are used automatically.

These permissions govern workspace-managed skills in ChatGPT. Skills used in other OpenAI products, including Codex, may be governed separately. For more information, see: [Using Codex with your ChatGPT plan](https://help.openai.com/articles/11369540).

# Manage workspace skills in the admin center

The admin **Skills** page is separate from the [**Permissions & roles**](https://chatgpt.com/admin/permissions) page. **Permissions & roles** controls which roles can create, use, upload, share, publish, or install skills. The admin **Skills** page helps workspace admins review and manage individual skills created in their workspace.  
  
When available for your workspace, go to the admin **Skills** page to review workspace skills. The table includes each skill's **Owner**, **Access**, **Users**, **Invocations (30d)**, **Created**, and **Updated** information. Admins can search skills, filter by **All**, **Invite only**, or **Workspace**, and sort by **Skill**, **Created**, or **Updated**.  
  
From the admin **Skills** page, admins can:

* Select a skill to view its details.
* Select the **+** (**Add skill**) button to upload a skill to the workspace.
* Select the more options menu (•••), then select **Download** to download a skill.
* Select **Change who has access** to update which users, groups, or workspace members can access the skill.
* Select **Change owner** to transfer ownership. The current owner keeps access after the transfer.
* Select **Delete skill** to permanently delete the skill from the workspace. People who currently have access will no longer be able to use it.

ChatGPT [Enterprise](https://chatgpt.com/business/enterprise/) and Edu admins can also manage how skills are shared and used across teams with:

* Compliance Platform: Eligible Enterprise and Edu administrators may review supported skill events and related conversation records when the platform is available and configured. For current guidance, see: [OpenAI Compliance Platform for Enterprise and Edu Customers](https://help.openai.com/articles/9261474).
* Data residency: Eligible workspace data-residency settings apply to supported, in-scope ChatGPT customer content. External services or resources used by a skill may have separate storage and processing terms.

Skills are supported by our [enterprise-grade programs](https://openai.com/business-data/). By default, data shared with a skill is not used to improve our models for ChatGPT business plans.
