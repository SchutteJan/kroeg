<script lang="ts">
	import { register } from '../api/session'

	function handleRegisterResponse(response: Response) {
		if (response.ok) {
			// TODO: Log the user in immediately
			alert('Registered successfully, continue to login')
			location.href = '/login'
		} else {
			// TODO: Handle responses
			alert('Register failed: ' + response.statusText)
		}
	}

	function handleSubmit(event: SubmitEvent) {
		if (!(event.target instanceof HTMLFormElement)) {
			return
		}
		event.submitter?.setAttribute('aria-busy', 'true')
		register(new FormData(event.target)).then(handleRegisterResponse)
	}
</script>

<form on:submit|preventDefault={handleSubmit}>
	<fieldset>
		<label>
			Email
			<input
				type="email"
				name="email"
				id="email"
				aria-label="Login"
				autocomplete="email"
				required
			/>
		</label>
		<label>
			Password
			<input
				type="password"
				name="password"
				aria-label="Password"
				autocomplete="new-password"
				required
				minlength="8"
			/>
		</label>
	</fieldset>
	<button type="submit">Register</button>
</form>
