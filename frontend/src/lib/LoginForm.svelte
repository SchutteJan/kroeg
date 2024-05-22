<script lang="ts">
	function handleLoginResponse(response: Response) {
		if (response.ok) {
			alert('Login successful');
		} else {
			// TODO: Handle responses
			alert('Login failed: ' + response.statusText);
		}
	}

	function handleSubmit(event: Event) {
		if (!(event.target instanceof HTMLFormElement)) {
			return;
		}
		event.preventDefault();
		fetch('/session/login', {
			method: 'POST',
			headers: {
				Accept: 'application/json'
			},
			body: new FormData(event.target)
		}).then(handleLoginResponse);
	}
</script>

<form on:submit|preventDefault={handleSubmit}>
	<fieldset>
		<label>
			Email
			<input type="text" name="email" placeholder="Email" aria-label="Login" required />
		</label>
		<label>
			Password
			<input
				type="password"
				name="password"
				placeholder="Password"
				aria-label="Password"
				required
				minlength="8"
			/>
		</label>
	</fieldset>
	<input type="submit" value="Login" />
</form>
