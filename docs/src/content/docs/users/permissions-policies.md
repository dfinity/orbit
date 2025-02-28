---
title: Permissions and Policies in Orbit
sidebar:
  label: Permissions & Policies
description: This page provide the necessary information to get started with Orbit.
---

Permissions in Orbit Wallet define access to various resources and operations within the platform. These permissions ensure secure, controlled interactions with key functionalities, enabling administrators to assign roles and responsibilities effectively.

## **Resources Managed by Permissions**

Orbit permissions control access to the following resource types:

- **User:** Manage user accounts and roles.
- **Group:** Organize and manage groups for efficient permission handling.
- **Account:** Manage digital asset accounts, including transactions and approvals.
- **Address Book:** Store and manage contact information for transactions.
- **Access Policy:** Define policies controlling access to resources.
- **Request Policy:** Set rules for request handling and approvals.
- **Request:** Manage individual requests for operations.
- **System:** Access and manage system configurations.
- **External Canister:** Manage and monitor external canisters (smart contracts).
- **Asset:** Manage supported digital assets.

## **Default Permission Actions**

The following actions can be defined for each resource:

- **List:** View a list of available resources.
- **Create:** Add new instances of the resource.
- **Read:** View details of the resource.
- **Update:** Modify existing resources.
- **Delete:** Remove resources.

### **Additional Actions for Specific Resources**

- **Transfer (Account):** Create transactions involving digital assets.
- **Fund (External Canister):** Add cycles to external canisters.

## **Assigning Permissions**

Permissions can be assigned at various levels to ensure flexibility and control:

### **Assignment Levels:**

1. **Group:** Assign permissions to predefined groups for easier management.
2. **Specific User:** Grant permissions to individual users.
3. **Everyone:** Define permissions that apply broadly to all users.

### **Broad permissions:**

- **Specific Users/Groups:** Accessible only to the selected users or groups.
- **All Logged-In Users:** Accessible by any user who is logged in.
- **Public (No Login Required):** Accessible by anyone, including non-logged-in visitors.

:::tip
Use group-based permissions for efficient management of large user bases.
:::

## **Managing Permissions**

### **Steps to View or Modify Permissions:**

1. Navigate to **Settings > User Groups & Permissions > Manage Permissions**.
2. Select the resource type you want to manage.
3. Assign actions (List, Create, Read, Update, Delete) based on user, group, or everyone levels.
4. Save changes to apply the updated permissions.

### **Best Practices:**

- **Least Privilege:** Grant only the necessary permissions to each user or group.
- **Regular Audits:** Periodically review permissions to ensure they align with organizational needs.

## **Example Scenarios**

### **Scenario 1: Multi-User Wallet Management**

- **Group:** Assign "Read" and "Transfer" permissions to the Finance Group for account resources.
- **Specific User:** Grant "Update" permission to the Compliance Officer.
- **Everyone:** Restrict access to "List" for account resources.

### **Scenario 2: External Canister Funding**

- **Group:** Assign "Fund" permissions to the Operations Group.
- **Specific User:** Allow "Read" access for monitoring purposes to a specific developer.
- **Everyone:** Deny public access to external canister management.

## **Understanding Approval Policies**

Approval policies are sets of rules that determine how transactions and actions within an Account are approved before execution. These policies can be predefined for convenience or defined manually on a per-action basis.

### **Benefits of Predefined Approval Policies:**

- **Consistency:** Apply the same set of rules across different actions.
- **Ease of Maintenance:** Update rules in one place and apply changes system-wide.
- **Improved Efficiency:** Quickly assign pre-configured rules to new approval policies.

## **Policy Rules**

Approval policies can be based on the following rule types:

- **Auto-Approved:** Requests are automatically approved without any user action.
- **Quorum Percentage:** Approval is granted when a specified percentage of approvers have approved the request.
- **Quorum:** Approval requires a defined minimum number of approvers.
- **All Of:** Every assigned approver must approve the request.
- **Any Of:** Approval is granted when any one of the assigned approvers approves the request.
- **None Of:** The request is automatically rejected, preventing execution.

## **Creating or Modifying Approval Policies**

### **Steps:**

1. Navigate to **Settings > Request Policies**.
2. Select the action for which you want to configure a policy.
3. Choose a predefined policy or assign the rule type (auto-approved, quorum, etc.) manually.
4. Configure any additional parameters, such as quorum size or percentage.
5. Save the policy to activate it.

## **Best Practices for Approval Policies**

- **Use Predefined policies for Consistency:** Define approval rules for common approval scenarios to reduce errors.
- **Leverage Quorum for High-Value Transactions:** Require multiple approvers for sensitive operations.
- **Avoid Over-Complexity:** Keep policies straightforward to avoid delays in approvals.
