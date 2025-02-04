---
title: Requests in Orbit Wallet
sidebar:
  label: Requests
description: This page provide the necessary information to get started with Orbit.
---

Requests are at the core of how Orbit Wallet processes transactions and actions. They represent operations such as asset transfers, permission changes, or wallet upgrades that require review and approval before execution.



## **What is a Request?**
A request is a formal operation submitted by a user to perform an action within Orbit Wallet, such as:
- **Sending assets**
- **Updating permissions**
- **Managing external canisters**
- **Performing bulk transactions**

Requests ensure secure and transparent collaboration by requiring review and approval based on your wallet's configured policies.



## **Creating Requests**

### **Steps to Create a Request:**
1. Navigate to the **Dashboard** and select the desired action (e.g., **Transfer Assets**).
2. Enter the required details (e.g., recipient address and transaction amount).
3. Click **"Create"** to submit the action request.

Transfer requests will appear in the **Transfer Requests** section and all requests are available in **Settings > Requests**.



## **Approving Requests**

### **Steps to Approve a Request:**
1. Navigate to **Transfer Requests** or **Settings > Requests**.
2. Select the request you wish to review.
3. Verify the request details, including recipient, amount, and transaction purpose.
4. Click **"Approve"** or **"Reject"** based on your assessment.
5. (If multi-signature approval is required) Wait for additional approvers to confirm the request.

:::note
Only users with appropriate permissions can approve requests.
:::



## **Diagnosing Rejected Requests**

### **Why Are Requests Rejected?**
- **Policy Violations:** The request doesn't meet the approval policy criteria.
- **Manual Rejection:** An approver rejected the request after reviewing it.

### **How to Diagnose Rejected Requests:**
1. Go to **Settings > Requests > Rejected**.
2. Review the rejection reason and comments (if provided).
3. Adjust the request parameters and resubmit if appropriate.



## **Request Modifications and Expiration**

### **Modifying a Request:**
- Requests cannot be directly edited after submission.
- If a request requires changes, it must be rejected and resubmitted with updated information.

### **Request Expiration:**
- Requests may have expiration times based on policy configurations.
- Expired requests cannot be approved and must be resubmitted if necessary.



## **Best Practices for Handling Requests**
- **Review Thoroughly:** Always double-check request details before approving.
- **Use Conditional Approval Policies:** Automate decisions for recurring request types.
- **Communicate with Requesters:** Provide clear reasons when rejecting requests to facilitate corrections.  
