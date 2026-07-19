import { expect, test } from '@playwright/test';

test('login, compose, send, find sent message, and move it to Archive', async ({ page }) => {
  const subject = `E2E sent mail ${Date.now()}`;
  const body = `This message was created by the e2e suite at ${new Date().toISOString()}.`;
  const recipient = 'recipient@example.com';

  await page.goto('/');
  await expect(page.getByRole('heading', { name: 'A quieter mailbox' })).toBeVisible();

  await page.getByRole('button', { name: 'Sign in' }).click();
  await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

  await page.getByRole('button', { name: 'Compose' }).click();
  await page.getByLabel('To').fill(recipient);
  await page.getByLabel('Subject').fill(subject);
  await page.getByLabel('Message').fill(body);
  await page.getByRole('button', { name: 'Send' }).click();
  await expect(page.getByLabel('Compose message')).toBeHidden();

  const mailboxNav = page.getByRole('navigation', { name: 'Primary navigation' });
  await mailboxNav.getByRole('button', { name: 'Sent' }).click();
  const sentMessage = page.getByRole('button', { name: new RegExp(escapeRegExp(subject)) });
  await expect(sentMessage).toBeVisible();

  await sentMessage.click();
  await expect(page.getByRole('heading', { name: subject })).toBeVisible();
  await expect(page.getByText(body)).toBeVisible();

  await page.locator('article').getByRole('button', { name: 'Archive' }).click();
  await expect(sentMessage).toHaveCount(0);

  await mailboxNav.getByRole('button', { name: 'Archive' }).click();
  await expect(page.getByRole('button', { name: new RegExp(escapeRegExp(subject)) })).toBeVisible();
});

function escapeRegExp(value: string) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
