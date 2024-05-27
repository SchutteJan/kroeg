<script lang="ts">
	import { register } from '../api/session';

	function handleRegisterResponse(response: Response) {
		if (response.ok) {
			// TODO: Log the user in immediately
			alert('Registered successfully, continue to login');
			location.href = '/login';
		} else {
			// TODO: Handle responses
			alert('Register failed: ' + response.statusText);
		}
	}

	function handleSubmit(event: Event) {
		if (!(event.target instanceof HTMLFormElement)) {
			return;
		}
		register(new FormData(event.target)).then(handleRegisterResponse);
	}
</script>

<form on:submit|preventDefault={handleSubmit}>
	<fieldset>
		<label>
			Email
			<input type="text" name="email" placeholder="Email" aria-label="Register" required />
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
	<input type="submit" value="Register" />
</form>
