# 🧹 Mailify

```text

                   _  _  _   __
 _ __ ___    __ _ (_)| |(_) / _| _   _
| '_ ` _ \  / _` || || || || |_ | | | |
| | | | | || (_| || || || ||  _|| |_| |
|_| |_| |_| \__,_||_||_||_||_|   \__, |
                                 |___/

```

## What is Mailify?

Mailify is an open-source email service that you can self-host, designed for rapid delivery of transactional emails.

## Why Did We Build Mailify?

Mailify was born out of our frustration with existing email service providers. We previously used platforms like [SendGrid](https://sendgrid.com/), [Mailgun](https://www.mailgun.com/), [Mailchimp](https://mailchimp.com/), and [Sendinblue](https://www.sendinblue.com/), but they presented several challenges:

- Many are not truly geared towards transactional emails, leading to delays in critical messages like login notifications.
- Most services cannot be hosted on-premise, limiting control over your email operations.
- As American companies, they are subject to the Patriot Act, which raises concerns about user data privacy.
- They often lack user-friendly templating tools, requiring technical assistance for simple edits, which adds to the workload of development teams.

## Should You Use It?

If you've struggled to find a reliable solution for transactional emails, then absolutely—Mailify is for you!

## Why Choose Mailify?

- **For Startups:**

  - If budget constraints prevent you from using expensive mailing tools, Mailify offers an open-source solution that allows you to send emails from your own SMTP server (or via affordable options like Amazon SES).
  - Save time on email template changes by empowering your Product Owner or non-technical team members to make updates directly.
  - Looking to add features? Contribute through a pull request and enhance the tool to fit your needs.

- **For Large Companies:**
  - If regulations prohibit you from using external services due to data privacy concerns, Mailify is the perfect in-house solution.
  - Gain access without the hassle of proxy restrictions that often block external services.
  - Customize your SMTP authentication process to meet company policies.
  - Ensure that the platform is user-friendly enough for non-technical team members, allowing managers to compose emails without needing technical support.

## 💡 Usage

Mailify is a simple service that renders your mjml template, interpolates the data and then sends it to a SMTP server.
If you want to see how to create your own template, take a look at the `/template` folder in this repository.

You then have several options for starting mailify. We recommend using Docker if you are on a amd64, i386 or arm64v8 architecture.
By doing the following, you'll be able to have a running server that will render and send your email.

```bash
docker run -d \
  --name mailify \
  -e SMTP__HOSTNAME=localhost \
  -e SMTP__PORT=25 \
  -e SMTP__USERNAME=optional \
  -e SMTP__PASSWORD=optional \
  -e SMTP__TLS_ENABLED=true \
  -e SMTP__ACCEPT_INVALID_CERT=false \
  -e TEMPLATE__TYPE=local \
  -e TEMPLATE__PATH=/templates \
  -p 3000:3000 \
  -v /path/to/your/templates:/templates:ro \
  jdrouet/mailify:latest
```

Once your server started, you can simply send an email using an `HTTP` request.

```bash
curl -X POST -v \
  -H "Content-Type: application/json" \
  --data '{"from":"alice@example.com","to":"bob@example.com","params":{"some":"data"}}' \
  http://localhost:3000/templates/the-name-of-your-template/json
```

You can also send attachments using a multipart request.

```bash
curl -X POST -v \
  -F attachments=@asset/cat.jpg \
  -F from=alice@example.com \
  -F to=bob@example.com \
  -F params='{"some":"data"}' \
  http://localhost:3000/templates/user-login/multipart
```

You can configure it with [some environment variable](./wiki/environment-variables.md) and can find more information in [this wiki](./wiki/template-provider.md).

If you some API specification, the Open API specification is also available on `/openapi.json` when Mailify is running.

To use it in production, we prepared a documentation on how to use Mailify with [Amazon Simple Email Service](./wiki/with-aws-ses.md).

## 🗑️ Uninstallation

Running the below command will globally uninstall the `mailify` binary.

```bash
cargo uninstall mailify
```

Remove the project repo

```bash
rm -rf /path/to/git/clone/mailify
```

## 🤝 How to contribute

We welcome contributions!

- Fork this repository;
- Create a branch with your feature: `git checkout -b my-feature`;
- Commit your changes: `git commit -m "feat: my new feature"`;
- Push to your branch: `git push origin my-feature`.

Once your pull request has been merged, you can delete your branch.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
