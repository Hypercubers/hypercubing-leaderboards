# Frontend

The frontend is a React project built with TypeScript and Vite. It fetches data via JSON routes exposed in `hsc-leaderboard\src\api\json.rs`, and uses non JSON routes defined in `hsc-leaderboard/routes.rs`.

## Setup
1. Install Node.js
2. `npm install` to install dependencies (ignored by `.gitignore`)
3. `npm run dev` to run the frontend on port `http://localhost:5173/`

## Folder Structure

```text
frontend/
├── node_modules/          # automatically generated from node
├── public/                # assets that can skip the build pipeline
├── src/                   # main source code
│   ├── assets/            # images
│   ├── components/        # reusable UI components
│   │   ├──table/          # folder to organize table based components
│   │   ├──ui/             # imported components from Shadcn
│   ├── hooks/             # custom React hooks
│   ├── lib/               # helper files for backend and utilities
│   ├── pages/             # folder to organize main pages of the website
│   ├── App.tsx            # main application entry point
├── .gitignore             # specifies intentionally untracked files
├── package.json           # project dependencies and scripts
└── README.md              # the file you are reading right now
```

## Style guide
Built with [tailwindcss](https://tailwindcss.com/). In general, try to copy existing style classes. Example: links should have `className="text-sidebar-primary hover:underline"`

Many components are imported from [shadcn](https://ui.shadcn.com/docs/components). See their documentation for how to import and use components.

### Changing project theme
The current theme is defined in `components.json`
```JSON
{
  "$schema": "https://ui.shadcn.com/schema.json",
  "style": "radix-luma",
  "rsc": false,
  "tsx": true,
  "tailwind": {
    "config": "",
    "css": "src/index.css",
    "baseColor": "mist",
    "cssVariables": true,
    "prefix": ""
  },
  "iconLibrary": "lucide",
  "rtl": false,
  "menuColor": "default",
  "menuAccent": "subtle",
  "aliases": {
    "components": "@/components",
    "utils": "@/lib/utils",
    "ui": "@/components/ui",
    "lib": "@/lib",
    "hooks": "@/hooks"
  },
  "registries": {
    "@reui": "https://reui.io/r/{style}/{name}.json"
  }
}
```

Head to [shadcn.com/create](https://ui.shadcn.com/create) to customize the look of all existing and future imported components. After tweaking it to your liking, click `Get Code` and go to the `Existing Project` tab. Then select `Theme only` and copy the npm command into a terminal here.

## Testing
There is no testing framework set up currently. Manually test components to the highest standards you can.

Before committing, use `npm run lint` to catch errors/warnings and then fix them.
