<script lang="ts">
	import { login } from '../api/session';

	function handleLoginResponse(response: Response) {
		if (response.ok) {
			location.href = '/';
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
		login(new FormData(event.target)).then(handleLoginResponse);
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
