<!-- src/routes/signup/+page.svelte -->
<script>
    let email = '';
    let password = '';
    let company_name = '';
  
    const handleSubmit = () => {
        // Handle signup logic here
        console.log('Signing up:', { email, password, company_name });
    };
</script>
  
<style>
    /* Add some styles here for your form */
    body {
        font-family: 'Arial', sans-serif; /* Change font for better readability */
        background-color: #f0f4f8; /* Light background color for contrast */
        margin: 0;
        padding: 0;
    }
    
    .form-container {
        max-width: 400px;
        margin: 50px auto; /* Center the form vertically with top margin */
        padding: 2em; /* Increase padding */
        background-color: white; /* White background for form */
        border: 1px solid #ccc;
        border-radius: 10px; /* More rounded corners */
        box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1); /* Subtle shadow for depth */
    }

    h1 {
        text-align: center; /* Center the heading */
        color: #333; /* Darker color for better contrast */
    }

    label {
        display: block; /* Make labels block elements for spacing */
        margin-bottom: 5px; /* Space between label and input */
        color: #666; /* Slightly lighter color for labels */
    }

    input {
        width: 100%; /* Full width inputs */
        padding: 10px; /* Padding inside inputs */
        margin-bottom: 20px; /* Space between inputs */
        border: 1px solid #ccc; /* Border style */
        border-radius: 5px; /* Rounded input corners */
        font-size: 16px; /* Increase font size for inputs */
    }

    input:focus {
        border-color: #007bff; /* Change border color on focus */
        outline: none; /* Remove default outline */
        box-shadow: 0 0 5px rgba(0, 123, 255, 0.5); /* Subtle shadow on focus */
    }

    button {
        width: 100%; /* Full width button */
        padding: 10px; /* Padding for the button */
        background-color: #007bff; /* Button background color */
        color: white; /* Button text color */
        border: none; /* Remove default border */
        border-radius: 5px; /* Rounded corners for the button */
        font-size: 16px; /* Increase button font size */
        cursor: pointer; /* Pointer cursor on hover */
        transition: background-color 0.3s ease; /* Transition for hover effect */
    }

    button:hover {
        background-color: #0056b3; /* Darker blue on hover */
    }
</style>
  
<div class="form-container">
    <h1>Signup</h1>
    <form on:submit|preventDefault={handleSubmit}>
        <label for="email">Email:</label>
        <input type="email" id="email" bind:value={email} required />
        
        <label for="company_name">Company Name:</label>
        <input type="text" id="company_name" bind:value={company_name} required />

        <label for="password">Password:</label>
        <input type="password" id="password" bind:value={password} required />
  
        <button type="submit">Signup</button>
    </form>
</div>
