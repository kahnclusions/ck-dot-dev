use leptos::*;

use crate::ui::{
    icons::{GeoTag, PersonIcon},
    Link, Pane, Stack, Terminal,
};

#[component]
pub fn PortfolioEntry(
    company: String,
    position: String,
    dates: String,
    location: String,
    #[prop(default = None)] image: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <Pane title={format!("{} :: {}", company, dates)} image=image>
            <Stack>
                <p class="font-ospx flex flex-row gap-3"><span><PersonIcon /> {position}</span> <span><GeoTag /> {location}</span></p>
                {children()}
            </Stack>
        </Pane>
    }
}

#[component]
pub fn PortfolioPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 mx-3">
            <Terminal title="portfolio".into()>
                <Stack>
                    <p>"I have experience scaling up new web apps from scratch to millions of users in production, I've deployed and maintained critical production systems, and I've lead dev teams and mentored engineers. I specialize in full stack development with React, Next.js, and Rust. I also have experience in Proxmox and managing a cluster of VMs using NixOS."</p>
                    <p>"I code in neovim and my dotfiles can be found "<Link href="https://github.com/kahnclusions/dotfiles.git".into() rel="noreferrer nofollow".into()>"here"</Link></p>
                </Stack>
            </Terminal>
            <Pane title="skills".into()>
                <div class="items-start text-left">
                    <p><strong>"Languages: "</strong>"TypeScript, JavaScript, Rust, Scala, Python, Lua, Nix"</p>
                    <p><strong>"Frameworks:"</strong>" React.js, Next.js, Solid.js, Node.js, Express, Koa, Feathers.js"</p>
                    <p><strong>"Databases:"</strong>" PostgreSQL, MySQL, Sqlite, MongoDB"</p>
                    <p><strong>"Cloud:"</strong>" Amazon AWS, Supabase, OpenAI integration, Proxmox 7, NixOS"</p>
                </div>
            </Pane>
            <PortfolioEntry
                company="Trait".into()
                position="Senior Software Engineer".into()
                dates="2023/8 - 2023/5".into()
                location="Taipei, Taiwan".into()
                image=Some("/images/wisepanda.jpg".into())
            >
                <p>At Trait I lead the frontend development of their latest product, a language learning platform for connecting students with verified Mandarin Chinese tutors.</p>
                <ul class="list-disc ml-5 space-y-1">
                    <li>Planning frontend architecture with Next.js 13/14 app directory</li>
                    <li>Making major contributions to the core frontend functionality using React and Next.js</li>
                    <li>Creating new API endpoints, implemented core business logic, and database access code.</li>
                    <li>Contributed to the overall system architecture and backend.</li>
                </ul>
                <p>My work has helped scale from nothing to 500+ tutors and 12k active users, and allows the company to scale way beyond this.</p>
                <p>Tech stack: Next.js 14 (app dir), React 18, Tailwind CSS, MongoDB, Node.js, Feathers.js, Playwright, React Testing Library, Heroku, AWS, OpenAI ChatGPT</p>
            </PortfolioEntry>
            <PortfolioEntry
                company="Intuit (Credit Karma UK)".into()
                position="Senior Software Engineer".into()
                dates="2021/3 - 2023/7".into()
                location="London, UK".into()
                image=Some("/images/creditkarma-dashboard.jpg".into())
            >
                <p>"I was a technical lead on Intuit Credit Karma's International team, contributing to "
                    <Link href="https://creditkarma.co.uk".into() rel="noreferrer nofollow noopener".into() newtab=true>creditkarma.co.uk</Link>
                " and "
                <Link href="https://creditkarma.ca".into() rel="noreferrer nofollow".into() newtab=true>creditkarma.ca</Link>
                " where I delivered features into production systems affecting our 2+ million UK members. My responsibilities included:"</p>
                <ul class="list-disc ml-5 space-y-1">
                    <li>Leading technical design on new projects including planning to delivery of a new AI-powered dashboard that increased conversion by showing more relevant content to members.</li>
                    <li>Working directly with stakeholders in my org, and in other orgs, to gather requirements and build the roadmaps and TDDs to deliver projects within timelines securely and robustly.</li>
                    <li>Mentoring developers, providing feedback and code review.</li>
                    <li>Implementing frontend features in React.js</li>
                    <li>Implementing backend APIs in Scala and Finagle.</li>
                    <li>Effecting positive change including testing and build strategy, type safety, and prototyping Next.js.</li>
                    <li>Response and incident management, root cause analysis.</li>
                    <li>Creating coding interview challenges and performing engineering interviews.</li>
                </ul>
                <p>"Tech stack: React, TypeScript, and urql (GraphQL) on frontend, with Scala Finagle, Hapi, Thrift on the backend. We deploy to Kubernetes with CircleCI and monitor with New Relic and Splunk. Some Next.js."</p>
            </PortfolioEntry>
            <PortfolioEntry
                company="Bulb Energy".into()
                position="Full Stack Software Engineer".into()
                dates="2019/12 - 2021/3".into()
                location="London, UK".into()
            >
                <p>"At Bulb I helped to launch the new Smart Pay-As-You-Go product with a squad of engineers, including doing the technical design, mentoring, and guiding the scrum process. I also lead a project to automate the generation of API documentation and client code. I also conducted technical interviews and helped improve the interview process."</p>
                <p>"The company sadly was one of many energy companies that didn't survive the UK energy crisis."</p>
                <p>"Technologies I used: TypeScript, React, React Native, Redux, GraphQL, Knex, Cypress, Kubernetes, Koa."</p>
            </PortfolioEntry>
            <PortfolioEntry
                company="YOOX Net-a-Porter Group".into()
                position="Senior Frontend Engineer".into()
                dates="2017/5 - 2019/12".into()
                location="London, UK".into()
                image=Some("/images/mrporter.jpg".into())
            >
                <p>
                    "At Net-a-Porter I lead a squad of 3-4 engineers to build a new shopping cart, checkout, and account applications for Net-a-Porter’s to a modern web stack on "
                    <Link href="https://www.mrpoter.com".into() rel="noreferrer noopener nofollow".into() newtab=true>"mrporter.com"</Link>
                    " and "
                    <Link href="https://www.theoutnet.com".into() rel="noreferrer noopener nofollow".into() newtab=true>"theoutnet.com"</Link>
                    ". I was a mentor of graduate scheme hires, ran technical interviews, and helped effect positive change to our ways of working and continuous delivery process."</p>
                <p>"My responsibilities included gathering and understanding product requirements to build up the team backlog, writing new features in React and JavaScript, leading scrum process for the team, running technical interviews."</p>
                <p>"I was also a mentor for fresh grad devs, I lead a migration to implement TypeScript and teach type-safe programming techniques, I implemented a migration to Cypress UI testing, and overall effected positive change in our ways of working and scrum processes, "</p>
                <p>"Tech stack: TypeScript, React, Redux, Cypress, Kubernetes"</p>
            </PortfolioEntry>
            <PortfolioEntry
                company="QuickSeries Publishing".into()
                position="Full Stack Engineer".into()
                dates="2015/10 - 2017/5".into()
                location="Montreal, Canada".into()
            >
                <p>"At YOOX Net-a-Porter Group I lead a squad of 3-4 engineers to build a new shopping cart, checkout, and account applications for Net-a-Porter’s to a modern web stack. I was a mentor of graduate scheme hires, ran technical interviews, and helped effect positive change to our ways of working and continuous delivery process."</p>
                <p>"Tech stack: TypeScript, React, Redux, Cypress, Kubernetes"</p>
            </PortfolioEntry>
            <PortfolioEntry
                company="ShiftFocus Media".into()
                position="Lead Web Developer".into()
                dates="2013/4 - 2015/10".into()
                location="Montreal, Canada".into()
            >
                <p>"I lead a small team of 3 web developers to build an online classroom platform for the Quebec Ministry of Education, as well as an account management platform for the Leadership Committee for English Education in Québec."</p>
                <p>"Tech stack: Scala, Play Framework, JavaScript, React, PostgreSQL, AWS"</p>
            </PortfolioEntry>
            <PortfolioEntry
                company="Coldfront Labs".into()
                dates="2012/6 - 2013/4".into()
                position="Web Developer".into()
                location="Ottawa, Canada (Remote)".into()
    >
                <p>"At Coldfront Labs I helped develop Drupal plugins and themes in PHP and JavaScript for the University of Ottawa student portal."</p>
                <p>"Tech stack: TypeScript, React, Redux, Cypress, Kubernetes"</p>
            </PortfolioEntry>
            <PortfolioEntry company="Imajery".into() dates="2011/3 - 2012/9".into() location="Montreal".into() position="Web Developer".into()>
                <p>"I helped Imajery building and launching boutique websites in WordPress and Magento Commerce. My responsibilities included interacting directly with clients to understand their needs, building and deploying websites, writing plugins and themes, maintaining existing sites. I also migrated the company to use Git code repository."</p>
                <p>"Tech stack: TypeScript, React, Redux, Cypress, Kubernetes"</p>
            </PortfolioEntry>
            <PortfolioEntry
                company="Ericsson".into()
                dates="2010/5 - 2010/12".into()
                position="Software Developer".into()
                location="Montreal, Canada".into()
    >
                <p>"I joined Ericsson for an 8-month internship as a part of my university studies. I worked on both a PHP web application and a backend web service in Java. My responsibilities including creating new features, fixing bugs, and helping the team transition to agile methodology."</p>
                <p>"Tech stack: JavaScript, PHP, MySQL, in-house web framework"</p>
            </PortfolioEntry>
        </div>
    }
}
