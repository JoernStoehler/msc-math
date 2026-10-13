# Reading view: chatgpt-custom-instructions

> Generated from the normalized source mirror. Immutable response snapshots are retained under `corpus/snapshots/`; this view is disposable and may be regenerated.

- Source ID: `chatgpt-custom-instructions`
- Source URL: https://help.openai.com/en/articles/8096356-chatgpt-custom-instructions
- Resolved URL: https://help.openai.com/en/articles/8096356-chatgpt-custom-instructions
- Correction links: 3

## Curated corrections relevant to this source

### Correction 4

**Wrong: “An API tool can update ChatGPT account Custom Instructions.”** OpenAI says there is no API for that account UI setting; API messages are a separate mechanism. Source: [Custom Instructions](https://help.openai.com/en/articles/8096356-chatgpt-custom-instructions); local mirror: `corpus/articles/openai/chatgpt/chatgpt-custom-instructions.md`.

[Open curated correction reference](../../../../references/corrections.md)

### Correction 10

**Wrong: “Remember this” updates global ChatGPT Custom Instructions.** Memory and Custom Instructions have distinct controls, and memory is selective. Sources: [Memory](https://help.openai.com/en/articles/8590148-memory-in-chatgpt), [Custom Instructions](https://help.openai.com/en/articles/8096356-chatgpt-custom-instructions); local mirrors in `corpus/articles/openai/chatgpt/`.

[Open curated correction reference](../../../../references/corrections.md)

### Correction 14

**Vendor-doc inconsistency: Custom Instructions update timing.** The article explicitly says updates apply to existing conversations, then its history FAQ says reflected only in future conversations. Preserve both statements, distinguish historical records from effective behavior, and don't claim live reload was independently verified. Source: https://help.openai.com/en/articles/8096356-chatgpt-custom-instructions ; local mirror: `corpus/articles/openai/chatgpt/chatgpt-custom-instructions.md`.

[Open curated correction reference](../../../../references/corrections.md)

## Source mirror

<!-- source-id:chatgpt-custom-instructions -->

# ChatGPT Custom Instructions

FAQ about ChatGPT Custom Instructions

Updated: 2 months ago

## General Information

Custom instructions allow you to share anything you'd like ChatGPT to consider in its response. Your custom instructions are applied immediately to all chats.  
  
Custom instructions are available on all plans on Web, Desktop, iOS, and Android.

# How your data is used

* You can edit or delete custom instructions at any time for future conversations.
* Your instructions won’t be shared with shared link viewers.
* If you use third party plug-ins, then the model may provide plug-in developers with relevant information from your instructions. Only use plug-ins that you trust, and don’t share any information you don’t want to give to third party plug-in developers.
* Important: Information from your use of custom instructions will also be used to improve model performance – like teaching the model how to adapt its responses to your instructions without overdoing it.
* [Learn more](https://help.openai.com/en/articles/5722486-how-your-data-is-used-to-improve-model-performance) about how we use your data and how you can opt out of your content being used to improve model performance.

# Enabling Custom Instructions

Updates to custom instructions settings are applied immediately across all chats (including existing conversations).).

## iOS & Android

1. In your Settings, select Customize ChatGPT.
2. Please make sure that Enable customization is toggled ON.
3. Enter your instructions in the Custom Instructions field.

## Web & Desktop

1. In your [Settings](https://chat.openai.com/#settings), select Personalization.
2. Please make sure that Enable customization is toggled ON.
3. Enter your instructions in the Custom Instructions field.

# Disabling Custom Instructions

## iOS & Android

1. In your Settings, select Customize ChatGPT.
2. Toggle Enable customization OFF.
3. *[Optional]* Delete your instructions from the relevant fields.

## Web & Desktop

1. In your [Settings](https://chat.openai.com/#settings), select Personalization and click on Custom Instructions.
2. Toggle Enable customization OFF.
3. *[Optional]* Delete your instructions from the relevant fields.

# Frequently asked questions

## What is the benefit of using custom instructions?

This feature allows customization of ChatGPT's responses based on your preferences, and can be modified or removed anytime for future conversations.

## Is there a character limit for custom instructions?

Free and Go users can save up to 1,500 characters in custom instructions. Plus, Pro, Enterprise, Business, and Education users can save up to 5,000 characters.

## Are custom instructions included in my ChatGPT data export?

Yes.

## Are my custom instructions data deleted when I delete my OpenAI account?

When you delete your OpenAI account, custom instructions that are tied to your account will also be deleted within 30 days as part of that process. If you haven't opted out of having your content used to improve our services, then we may have chat records that are de-identified and disassociated from your account.

## If I update or remove my custom instructions, will previous versions of my instructions continue to appear in my chat history?

Yes, updates to your instructions are reflected only in future conversations. To remove custom instructions from previous conversations, you can clear those conversations from your chat history. Learn more about clearing conversations in [this Help Center article](https://help.openai.com/en/articles/6378407-how-can-i-delete-my-account#h_c4bbbd07c5).

## Chat Completions API system message vs Custom Instructions in UI

There will be no API for custom instructions, as the Chat Completions API system messages should be used for a similar effect. [Learn more](https://help.openai.com/en/articles/8234522-chat-completions-api-system-message-vs-custom-instructions-in-ui).
