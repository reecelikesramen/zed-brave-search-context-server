# Zed Brave Search Context Server

This extension provides a Model Context Server for Brave Search, for use with the Zed AI assistant.

It adds `/brave_web_search` and `/brave_local_search` slash commands to the Assistant Panel.

## Configuration

### Getting an API Key

To use the extension, first you will need to acquire a Brave API key.

1. Sign up for a [Brave Search API account](https://brave.com/search/api/)
2. Choose a plan (Free tier available with 2,000 queries/month)
3. Generate your API key [from the developer dashboard](https://api.search.brave.com/app/keys)



```json
{
  "context_servers": {
    "brave_search": {
      "settings": {
        "brave_api_key": "<your brave API key>"
      }
    }
  }
}
```

## Usage

- `/brave_web_search <query> [count] [offset]`: Execute web searches with pagination and filtering.
- `/brave_local_search <query> [count]`: Search for local businesses and services. Automatically falls back to web search if no local results found.
